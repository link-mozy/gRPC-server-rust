use std::str::FromStr;
use tonic::{Request, Response, Status};
use calc_program::{SumRequest, SumResponse, MinusRequest, MinusResponse, calc_program_server::CalcProgram};

pub mod calc_program {
    tonic::include_proto!("calc_program");
}

#[derive(Debug, Default)]
pub struct CalcProgramService {}

#[tonic::async_trait]
impl CalcProgram for CalcProgramService {
    async fn sum(
        &self,
        request: Request<SumRequest>
    ) -> Result<Response<SumResponse>, Status> {
        let req = request.into_inner();
        let number_1:i32 = i32::from_str(&req.number_1).unwrap();
        let number_2:i32 = i32::from_str(&req.number_2).unwrap();
        let result: i32 = number_1 + number_2;

        Ok(Response::new(calc_program::SumResponse {
            data: {result.to_string()}
        }))
    }

    async fn minus(
        &self,
        request: Request<MinusRequest>
    ) -> Result<Response<MinusResponse>, Status> {
        let req = request.into_inner();
        let number_1:i32 = i32::from_str(&req.number_1).unwrap();
        let number_2:i32 = i32::from_str(&req.number_2).unwrap();
        let result: i32 = number_1 - number_2;

        Ok(Response::new(calc_program::MinusResponse {
            data: {result.to_string()}
        }))
    }
}