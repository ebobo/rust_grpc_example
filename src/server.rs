use tonic::{transport::Server, Request, Response, Status};
use proto::{UnitId, Unit, UnitList, Property};
use proto::unit_service_server::{UnitService, UnitServiceServer};

pub mod proto {
    tonic::include_proto!("proto.v1");
}


#[derive(Debug, Default)]
pub struct UnitServiceServerImpl {}

#[tonic::async_trait]
impl UnitService for UnitServiceServerImpl {
    async fn get_property(&self,request: Request<UnitId>) -> Result<Response<Property>, Status>  {
        println!("Got a request: {:?}", request);

        let reply = Property {
            text: "63°26'48''N, 10°23'55''E".to_string(),
            role: "GPS Position".to_string(),
            r#type: "coordinates".to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn list_units(&self,_request: Request<()>) -> Result<Response<UnitList>, Status>{
        let mut unit_list = UnitList::default();

        // Simulating a list of units
        let units = vec![
            Unit {
                id: Some(UnitId {
                    namespace: "AM4".to_string(),
                    label: "Position".to_string(),
                }),
                property: Some(Property {
                    text: "63°26'48''N, 10°23'55''E".to_string(),
                    role: "GPS Position".to_string(),
                    r#type: "coordinates".to_string(),
                }),
            },
            Unit {
                id: Some(UnitId {
                    namespace: "AM4".to_string(),
                    label: "Course".to_string(),
                }),
                property: Some(Property {
                    text: "30°W".to_string(),
                    role: "Vessel Course".to_string(),
                    r#type: "heading".to_string(),
                }),
            },
            Unit {
                id: Some(UnitId {
                    namespace: "AM4".to_string(),
                    label: "Deviation".to_string(),
                }),
                property: Some(Property {
                    text: "1°E".to_string(),
                    role: "Vessel Deviation".to_string(),
                    r#type: "heading".to_string(),
                }),
            },
        ];

        unit_list.units = units;
        Ok(Response::new(unit_list))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:9086".parse()?;
    let btc_service = UnitServiceServerImpl::default();
    
    println!("Rust gRPC Server is running on {}", addr);
    
    Server::builder()
    .add_service(UnitServiceServer::new(btc_service))
    .serve(addr)
    .await?;
    
    Ok(())
}