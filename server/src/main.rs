use anyhow::Result;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    let good_json = serde_json::json!({ "bobo": { "job": "clown", "friendly": true } });

    let bad_json = serde_json::json!({ "number": 1 });

    let good_doc = {
        let initial_state = automerge::Value::from_json(&good_json);
        let (_, change) = automerge::Frontend::new_with_initial_state(initial_state)?;
        let mut backend = automerge::Backend::init();
        backend.apply_local_change(change)?;
        backend.save()?
    };

    let bad_doc = {
        let initial_state = automerge::Value::from_json(&bad_json);
        let (_, change) = automerge::Frontend::new_with_initial_state(initial_state)?;
        let mut backend = automerge::Backend::init();
        backend.apply_local_change(change)?;
        backend.save()?
    };

    let good_doc_route = warp::path!("good_doc")
        .and(warp::get())
        .and(warp::any().map(move || good_doc.clone()))
        .map(|doc: Vec<u8>| warp::http::Response::builder().status(200).body(doc));

    let bad_doc_route = warp::path!("bad_doc")
        .and(warp::get())
        .and(warp::any().map(move || bad_doc.clone()))
        .map(|doc: Vec<u8>| warp::http::Response::builder().status(200).body(doc));

    let routes = good_doc_route.or(bad_doc_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;
    Ok(())
}
