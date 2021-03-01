use rover_postcard_protocol::Driver;
use rover_tonic::borealis::kinematic_arm_state_servicer_server::{
    KinematicArmStateServicer,
};
use rover_tonic::borealis::{ArmState, Empty};
use tokio_serial::{DataBits, FlowControl, Parity, Serial, SerialPortSettings, StopBits};
use tonic::{ Request, Response, Status};

use tracing::debug;

pub struct KinematicArmServer {
    driver: Driver,
}

impl KinematicArmServer {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            driver: Driver::new(Serial::from_path(
                path,
                &SerialPortSettings {
                    baud_rate: 9600,
                    data_bits: DataBits::Eight,
                    flow_control: FlowControl::None,
                    parity: Parity::None,
                    stop_bits: StopBits::One,
                    timeout: Default::default(),
                },
            )?),
        })
    }
}

#[tonic::async_trait]
impl KinematicArmStateServicer for KinematicArmServer {
    async fn get_arm_state(&self, _request: Request<Empty>) -> Result<Response<ArmState>, Status> {
        let hardware_response = self
            .driver
            .do_hardware_action(rover_postcard_protocol::rover_postcards::Request {
                kind: rover_postcard_protocol::rover_postcards::RequestKind::GetKinematicArmPose,
                state: rand::random(),
            })
            .await
            // need to manually map the error type as they arn't compatible
            .map_err(|_| Status::aborted("failed to interrogate model arm."))?;
        debug!("Response := {:?}", hardware_response);
        if let Some(rover_postcard_protocol::rover_postcards::ResponseKind::KinematicArmPose(
            pose,
        )) = hardware_response.data
        {
            Ok(Response::new(ArmState {
                lower_axis: pose.lower_axis.unwrap_or(0.0),
                upper_axis: pose.upper_axis.unwrap_or(0.0),
                rotation: pose.rotation_axis.unwrap_or(0.0),
                gripper: None,
                driving_arm: true,
                driving_gripper: false,
            }))
        } else {
            Err(Status::aborted("invalid response from model arm hardware."))
        }
    }
}
