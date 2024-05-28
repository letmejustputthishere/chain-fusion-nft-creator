use super::distribution::SVG_COLOR_KEYWORDS;
use super::distribution::WEIGHTS;
use crate::{evm_rpc::RpcServices, storage::store_asset};
use ethers_core::types::U256;
use rand::distributions::{Distribution, WeightedIndex};
use rand_chacha::ChaCha20Rng;
use resvg::tiny_skia;
use resvg::usvg::fontdb::Database;
use resvg::usvg::Options;
use resvg::usvg::Tree;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_vec};
use svg::node::element::Circle;
use svg::node::element::Rectangle;
use svg::Document;

#[derive(Serialize)]
pub struct Attributes {
    pub bg_color: String,
    pub frame_color: String,
    pub circle_color: String,
}

#[derive(Serialize, Deserialize)]
struct Trait {
    pub trait_type: String,
    pub value: String,
}

impl Attributes {
    fn to_trait_list(&self) -> Vec<Trait> {
        serde_json::to_value(self)
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
            .into_iter()
            .map(|(k, v)| Trait {
                trait_type: k,
                value: v.as_str().unwrap().to_string(),
            })
            .collect()
    }
}

fn select_value<T>(values: &[T], weights: &[u32], rng: &mut ChaCha20Rng) -> T
where
    T: Clone,
{
    let dist = WeightedIndex::new(weights).unwrap();
    values[dist.sample(rng)].clone()
}

pub fn generate_attributes(rng: &mut ChaCha20Rng) -> Attributes {
    Attributes {
        bg_color: select_value(&SVG_COLOR_KEYWORDS, &WEIGHTS, rng).to_string(),
        frame_color: select_value(&SVG_COLOR_KEYWORDS, &WEIGHTS, rng).to_string(),
        circle_color: select_value(&SVG_COLOR_KEYWORDS, &WEIGHTS, rng).to_string(),
    }
}

pub fn generate_and_store_image(token_id: U256, attributes: &Attributes) {
    let bg = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", attributes.bg_color.clone());

    let frame = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "none")
        .set("stroke", attributes.frame_color.clone())
        .set("stroke-width", 20);

    let circle = Circle::new()
        .set("cx", 500)
        .set("cy", 500)
        .set("r", 480)
        .set("fill", attributes.circle_color.clone());

    let document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000))
        .add(bg)
        .add(frame)
        .add(circle);

    // Serialize the SVG document to String
    let svg_data = document.to_string();

    // Parse the SVG.
    let opt = Options::default();
    let font_db = Database::default();
    let tree = Tree::from_str(&svg_data, &opt, &font_db).expect("Failed to parse SVG.");

    // Render the SVG.
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let byte_vec = pixmap.encode_png().expect("Failed to encode PNG.");

    store_asset(
        format!("/{}.png", token_id),
        crate::storage::Asset {
            headers: vec![(String::from("Content-Type"), String::from("image/png"))],
            body: byte_vec,
        },
    );
    store_asset(
        format!("/{}.svg", token_id),
        crate::storage::Asset {
            headers: vec![(String::from("Content-Type"), String::from("image/svg+xml"))],
            body: svg_data.into_bytes(),
        },
    );
}

fn generate_image_url(token_id: U256, file_extension: String) -> String {
    // get RpcService from state
    let rpc_services = crate::state::read_state(|s| s.rpc_services.clone());
    let is_local_rpc = match rpc_services {
        RpcServices::Custom {
            chainId: _,
            services,
        } => {
            let mut found_local = false;
            for service in services {
                if service.url.contains("localhost") {
                    found_local = true;
                    break;
                }
            }
            found_local
        }
        _ => false,
    };
    if is_local_rpc {
        format!(
            "http://{}.localhost:4943/{}.{}",
            ic_cdk::id().to_text(),
            &token_id,
            file_extension
        )
    } else {
        format!(
            "https://{}.raw.icp0.io/{}.{}",
            ic_cdk::id().to_text(),
            &token_id,
            file_extension
        )
    }
}

pub fn generate_and_store_metadata(token_id: U256, attributes: &Attributes) {
    // create JSON metadata with serde_json
    let metadata = json!({
        "name": format!("Chainfusion #{}", token_id),
        "image": generate_image_url(token_id,"png".to_string()),
        "image_svg": generate_image_url(token_id,"svg".to_string()),
        "attributes" : attributes.to_trait_list(),
    });
    // Serialize the JSON value to a Vec<u8>
    let byte_vec = to_vec(&metadata).expect("json should be serializable to byte vector");

    store_asset(
        format!("/{}", token_id),
        crate::storage::Asset {
            headers: vec![(String::from("Content-Type"), String::from("text/json"))],
            body: byte_vec,
        },
    );
}
