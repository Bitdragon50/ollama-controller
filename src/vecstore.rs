use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, ScalarQuantizationBuilder,
    SearchParamsBuilder, SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant, QdrantError};
use tokio::runtime::Runtime;


pub fn save_embedding() -> Result<(), QdrantError> {
    let rt = Runtime::new().unwrap();
    // Example of top level client
    // You may also use tonic-generated client from `src/qdrant.rs`
    let client = Qdrant::from_url("http://localhost:6334").build()?;

    let collections_list = rt.block_on(client.list_collections())?;
    dbg!(collections_list);
    // collections_list = {
    //   "collections": [
    //     {
    //       "name": "test"
    //     }
    //   ]
    // }

    let collection_name = "test";
    rt.block_on(client.delete_collection(collection_name))?;

    rt.block_on(client
        .create_collection(
            CreateCollectionBuilder::new(collection_name)
                .vectors_config(VectorParamsBuilder::new(10, Distance::Cosine))
                .quantization_config(ScalarQuantizationBuilder::default()),
        ))?;

    let collection_info = rt.block_on(client.collection_info(collection_name))?;
    dbg!(collection_info);

    let payload: Payload = serde_json::json!(
        {
            "foo": "Bar",
            "bar": 12,
            "baz": {
                "qux": "quux"
            }
        }
    )
    .try_into()
    .unwrap();

    let points = vec![PointStruct::new(0, vec![12.; 10], payload)];
    rt.block_on(client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points)))?;

    let search_result = rt.block_on(client
        .search_points(
            SearchPointsBuilder::new(collection_name, [11.; 10], 10)
                .filter(Filter::all([Condition::matches("bar", 12)]))
                .with_payload(true)
                .params(SearchParamsBuilder::default().exact(true)),
        ))?;
    dbg!(&search_result);
    // search_result = [
    //   {
    //     "id": 0,
    //     "version": 0,
    //     "score": 1.0000001,
    //     "payload": {
    //       "bar": 12,
    //       "baz": {
    //         "qux": "quux"
    //       },
    //       "foo": "Bar"
    //     }
    //   }
    // ]

    let found_point = search_result.result.into_iter().next().unwrap();
    let mut payload = found_point.payload;
    let baz_payload = payload.remove("baz").unwrap().into_json();
    println!("baz: {}", baz_payload);
    // baz: {"qux":"quux"}

    Ok(())
}