use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, ScalarQuantizationBuilder,
    SearchParamsBuilder, SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder, Vectors,
};
use qdrant_client::{Payload, Qdrant, QdrantError};
use tokio::runtime::Runtime;


pub async fn save_embedding<s: Into<String > + Clone>(embeddings: Vec<Vec<f32>>, text: Vec<String>, store_name: s, dimensions: u64) -> Result<(), QdrantError> {
    let client = Qdrant::from_url("http://localhost:6334").build()?;
    if !client.collection_exists(store_name.clone()).await? {
        let collection_builder = CreateCollectionBuilder::new(store_name.clone()).vectors_config(VectorParamsBuilder::new(dimensions, Distance::Cosine));
        client.create_collection(collection_builder).await?;
    } else {
        client.delete_collection(store_name.clone()).await?;
        let quantize = ScalarQuantizationBuilder::default().build();
        let collection_builder = CreateCollectionBuilder::new(store_name.clone()).vectors_config(VectorParamsBuilder::new(dimensions, Distance::Cosine))
        .quantization_config(quantize);
        client.create_collection(collection_builder).await?;
    }

    let points = embeddings.into_iter().enumerate().map(|(index, embedding)| {
        let message = text[index].as_str();
        PointStruct::new(index as u64, embedding, [("question", message.into())])
    }).collect::<Vec<PointStruct>>();
    

    let response = client
    .upsert_points(UpsertPointsBuilder::new(store_name.clone(), points).wait(true))
    .await?;

    dbg!(response);

    //println!("{:#?}", test_collection_exist);
    Ok(())
}

pub async fn find_sim<S: Into<String > + Clone>(vector: Vec<f32>, store_name: S) -> Result<(), QdrantError> {
    let client = Qdrant::from_url("http://localhost:6334").build()?;

    let search_result = client
        .search_points(
            SearchPointsBuilder::new(store_name.clone(), vector, 2).with_payload(true),
        )
        .await?;

    dbg!(search_result);
    Ok(())
}