use proto::UnitId;
use proto::unit_service_client::UnitServiceClient;

pub mod proto {
    tonic::include_proto!("proto.v1");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UnitServiceClient::connect("http://[::1]:9086").await?;

    let request = tonic::Request::new(
        UnitId {
            namespace: "AM4".to_string(),
            label: "Position".to_string(),
        }
    );

    let response = client.get_property(request).await?;

    println!("Get Unit {:?}", response);

    let request2 = tonic::Request::new({});

    let response2 = client.list_units(request2).await?;
    
    println!("List Units {:?}", response2);

    Ok(())
}