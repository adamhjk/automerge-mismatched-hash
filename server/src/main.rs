use anyhow::Result;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    let good_json = serde_json::json!({ "bobo": { "job": "clown", "friendly": true } });

    let bad_json = serde_json::json!(
    {
      "id": "entity:56021541879545914",
      "name": "descriptive-meeting-0524",
      "objectType": "kubernetesDeployment",
      "description": "descriptive-meeting-0524",
      "expressionProperties": { "__baseline": {} },
      "manualProperties": { "__baseline": {} },
      "inferredProperties": {
        "__baseline": {
          "kubernetesObject": {
            "kind": "Deployment",
            "spec": {
              "replicas": 1,
              "selector": { "matchLabels": { "app": "descriptive-meeting-0524" } },
              "template": {
                "metadata": { "labels": { "app": "descriptive-meeting-0524" } }
              }
            },
            "metadata": { "name": "descriptive-meeting-0524-deployment" },
            "apiVersion": "apps/v1"
          }
        }
      },
      "properties": {
        "__baseline": {
          "kubernetesObject": {
            "kind": "Deployment",
            "spec": {
              "replicas": 1,
              "selector": { "matchLabels": { "app": "descriptive-meeting-0524" } },
              "template": {
                "metadata": { "labels": { "app": "descriptive-meeting-0524" } }
              }
            },
            "metadata": { "name": "descriptive-meeting-0524-deployment" },
            "apiVersion": "apps/v1"
          },
          "kubernetesObjectYaml": "apiVersion: apps/v1\nkind: Deployment\nmetadata:\n  name: descriptive-meeting-0524-deployment\nspec:\n  selector:\n    matchLabels:\n      app: descriptive-meeting-0524\n  replicas: 1\n  template:\n    metadata:\n      labels:\n        app: descriptive-meeting-0524\n"
        }
      },
      "nodeId": "node:56021541829214265",
      "head": false,
      "base": false,
      "siStorable": {
        "typeName": "entity",
        "objectId": "entity:56021541879545914",
        "billingAccountId": "billingAccount:56014734046003223",
        "organizationId": "organization:56014739397935131",
        "workspaceId": "workspace:56014739406323740",
        "tenantIds": [
          "billingAccount:56014734046003223",
          "organization:56014739397935131",
          "workspace:56014739406323740",
          "entity:56021541879545914"
        ],
        "createdByUserId": null,
        "updateClock": { "epoch": 0, "updateCount": 50 },
        "deleted": false
      },
      "siChangeSet": {
        "changeSetId": "changeSet:56014879538020398",
        "editSessionId": "editSession:56014879571574831",
        "event": "create",
        "orderClock": { "epoch": 0, "updateCount": 5 }
      }
    });

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
