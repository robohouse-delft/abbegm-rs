#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmHeader {
    /// sequence number (to be able to find lost messages)
    #[prost(uint32, optional, tag="1")]
    pub seqno: ::core::option::Option<u32>,
    /// controller send time stamp in ms
    #[prost(uint32, optional, tag="2")]
    pub tm: ::core::option::Option<u32>,
    #[prost(enumeration="egm_header::MessageType", optional, tag="3", default="MsgtypeUndefined")]
    pub mtype: ::core::option::Option<i32>,
}
/// Nested message and enum types in `EgmHeader`.
pub mod egm_header {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageType {
        MsgtypeUndefined = 0,
        /// for future use
        MsgtypeCommand = 1,
        /// sent by robot controller
        MsgtypeData = 2,
        /// sent by sensor for position guidance
        MsgtypeCorrection = 3,
        /// sent by sensor for path correction
        MsgtypePathCorrection = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmCartesian {
    #[prost(double, required, tag="1")]
    pub x: f64,
    #[prost(double, required, tag="2")]
    pub y: f64,
    #[prost(double, required, tag="3")]
    pub z: f64,
}
// If you have pose input, i.e. not joint input, you can choose to send orientation data as quaternion or as Euler angles.
// If both are sent, Euler angles have higher priority.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmQuaternion {
    #[prost(double, required, tag="1")]
    pub u0: f64,
    #[prost(double, required, tag="2")]
    pub u1: f64,
    #[prost(double, required, tag="3")]
    pub u2: f64,
    #[prost(double, required, tag="4")]
    pub u3: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmEuler {
    #[prost(double, required, tag="1")]
    pub x: f64,
    #[prost(double, required, tag="2")]
    pub y: f64,
    #[prost(double, required, tag="3")]
    pub z: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmClock {
    #[prost(uint64, required, tag="1")]
    pub sec: u64,
    #[prost(uint64, required, tag="2")]
    pub usec: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmPose {
    #[prost(message, optional, tag="1")]
    pub pos: ::core::option::Option<EgmCartesian>,
    #[prost(message, optional, tag="2")]
    pub orient: ::core::option::Option<EgmQuaternion>,
    #[prost(message, optional, tag="3")]
    pub euler: ::core::option::Option<EgmEuler>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmCartesianSpeed {
    #[prost(double, repeated, packed="false", tag="1")]
    pub value: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmJoints {
    #[prost(double, repeated, packed="false", tag="1")]
    pub joints: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmExternalJoints {
    #[prost(double, repeated, packed="false", tag="1")]
    pub joints: ::prost::alloc::vec::Vec<f64>,
}
/// Is used for position streaming (source: controller) and position guidance (source: sensor)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmPlanned {
    #[prost(message, optional, tag="1")]
    pub joints: ::core::option::Option<EgmJoints>,
    #[prost(message, optional, tag="2")]
    pub cartesian: ::core::option::Option<EgmPose>,
    #[prost(message, optional, tag="3")]
    pub external_joints: ::core::option::Option<EgmJoints>,
    #[prost(message, optional, tag="4")]
    pub time: ::core::option::Option<EgmClock>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmSpeedRef {
    #[prost(message, optional, tag="1")]
    pub joints: ::core::option::Option<EgmJoints>,
    #[prost(message, optional, tag="2")]
    pub cartesians: ::core::option::Option<EgmCartesianSpeed>,
    #[prost(message, optional, tag="3")]
    pub external_joints: ::core::option::Option<EgmJoints>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmPathCorr {
    /// Sensor measurement (x, y, z) relative the sensor tool coordinate system
    #[prost(message, required, tag="1")]
    pub pos: EgmCartesian,
    /// Sensor measurement age in ms
    #[prost(uint32, required, tag="2")]
    pub age: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmFeedBack {
    #[prost(message, optional, tag="1")]
    pub joints: ::core::option::Option<EgmJoints>,
    #[prost(message, optional, tag="2")]
    pub cartesian: ::core::option::Option<EgmPose>,
    #[prost(message, optional, tag="3")]
    pub external_joints: ::core::option::Option<EgmJoints>,
    #[prost(message, optional, tag="4")]
    pub time: ::core::option::Option<EgmClock>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmMotorState {
    #[prost(enumeration="egm_motor_state::MotorStateType", required, tag="1")]
    pub state: i32,
}
/// Nested message and enum types in `EgmMotorState`.
pub mod egm_motor_state {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MotorStateType {
        MotorsUndefined = 0,
        MotorsOn = 1,
        MotorsOff = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmMciState {
    #[prost(enumeration="egm_mci_state::MciStateType", required, tag="1", default="MciUndefined")]
    pub state: i32,
}
/// Nested message and enum types in `EgmMCIState`.
pub mod egm_mci_state {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MciStateType {
        MciUndefined = 0,
        MciError = 1,
        MciStopped = 2,
        MciRunning = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmRapidCtrlExecState {
    #[prost(enumeration="egm_rapid_ctrl_exec_state::RapidCtrlExecStateType", required, tag="1", default="RapidUndefined")]
    pub state: i32,
}
/// Nested message and enum types in `EgmRapidCtrlExecState`.
pub mod egm_rapid_ctrl_exec_state {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RapidCtrlExecStateType {
        RapidUndefined = 0,
        RapidStopped = 1,
        RapidRunning = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmTestSignals {
    #[prost(double, repeated, packed="false", tag="1")]
    pub signals: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmMeasuredForce {
    #[prost(double, repeated, packed="false", tag="1")]
    pub force: ::prost::alloc::vec::Vec<f64>,
}
/// Robot controller outbound message, sent from the controller to the sensor during position guidance and position streaming
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmRobot {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<EgmHeader>,
    #[prost(message, optional, tag="2")]
    pub feed_back: ::core::option::Option<EgmFeedBack>,
    #[prost(message, optional, tag="3")]
    pub planned: ::core::option::Option<EgmPlanned>,
    #[prost(message, optional, tag="4")]
    pub motor_state: ::core::option::Option<EgmMotorState>,
    #[prost(message, optional, tag="5")]
    pub mci_state: ::core::option::Option<EgmMciState>,
    #[prost(bool, optional, tag="6")]
    pub mci_convergence_met: ::core::option::Option<bool>,
    #[prost(message, optional, tag="7")]
    pub test_signals: ::core::option::Option<EgmTestSignals>,
    #[prost(message, optional, tag="8")]
    pub rapid_exec_state: ::core::option::Option<EgmRapidCtrlExecState>,
    #[prost(message, optional, tag="9")]
    pub measured_force: ::core::option::Option<EgmMeasuredForce>,
    #[prost(double, optional, tag="10")]
    pub utilization_rate: ::core::option::Option<f64>,
}
/// Robot controller inbound message, sent from sensor to the controller during position guidance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmSensor {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<EgmHeader>,
    #[prost(message, optional, tag="2")]
    pub planned: ::core::option::Option<EgmPlanned>,
    #[prost(message, optional, tag="3")]
    pub speed_ref: ::core::option::Option<EgmSpeedRef>,
}
/// Robot controller inbound message, sent from sensor during path correction
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgmSensorPathCorr {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<EgmHeader>,
    #[prost(message, optional, tag="2")]
    pub path_corr: ::core::option::Option<EgmPathCorr>,
}
