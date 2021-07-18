use std::os::raw::c_int;
use std::ffi::c_void;
use std::os::raw::c_char;

/* Encapuslation -> ibv_create_qp in C to Rust */

//#include <infiniband/verbs.h>
//struct ibv_qp ibv_create_qp(struct ibv_pd pd, struct ibv_qp_init_attr *qp_init_attr);
// --> create a queue pair(QP) associated with the protection domain (pd)


pub mod ibv_node_type {
    pub type Type = ::std::os::raw::c_int;
    pub const IBV_NODE_UNKNOWN: Type     = -1;
	pub const IBV_NODE_CA: Type          = 1;
	pub const IBV_NODE_SWITCH: Type      = 2;
	pub const IBV_NODE_ROUTER: Type      = 3;
	pub const IBV_NODE_RNIC: Type        = 4;
	pub const IBV_NODE_USNIC: Type       = 5;
	pub const IBV_NODE_USNIC_UDP: Type   = 6;
	pub const IBV_NODE_UNSPECIFIED: Type = 7;
}

pub mod ibv_transport_type {
    pub type Type = ::std::os::raw::c_int;
	pub const IBV_TRANSPORT_UNKNOWN: Type     = -1;
	pub const IBV_TRANSPORT_IB: Type          = 0;
	pub const IBV_TRANSPORT_IWARP: Type       = 1;
	pub const IBV_TRANSPORT_USNIC: Type       = 2;
	pub const IBV_TRANSPORT_USNIC_UDP: Type   = 3;
	pub const IBV_TRANSPORT_UNSPECIFIED: Type = 4;
}

// struct _ibv_device_ops {
// 	struct ibv_context *	(*_dummy1)(struct ibv_device *device, int cmd_fd);
// 	void			(*_dummy2)(struct ibv_context *context);
// };
#[repr(C)]
pub struct _ibv_device_ops {
    pub _dummy1: Option<unsafe extern "C" fn(device: *mut ibv_device, cmd_fd: c_int) -> *mut ibv_context>,
    pub _dummy2: Option<unsafe extern "C" fn(context: *mut ibv_context)>,
}
// TODO: figure it out!!!!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#[repr(C)]
pub struct ibv_device {
    _ops: _ibv_device_ops,
    node_type: ibv_node_type::Type,
    transport_type: ibv_transport_type::Type,
    name: [c_char; 64usize], // char name[64];
    dev_name: [c_char; 64usize],
    dev_path: [c_char; 256usize],
    ibdev_path: [c_char; 256usize],
}

#[repr(C)]
pub struct _compat_ibv_port_attr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ibv_mw {
    pub context: *mut ibv_context,
    pub pd: *mut ibv_pd,
    pub rkey: u32,
    pub handle: u32,
    pub type_: ibv_mw_type::Type,
}

pub mod ibv_mw_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const IBV_MW_TYPE_1: Type = 1;
    pub const IBV_MW_TYPE_2: Type = 2;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibv_mw_bind_info {
    pub mr: *mut ibv_mr,
    pub addr: u64,
    pub length: u64,
    pub mw_access_flags: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct ibv_mw_bind {
    pub wr_id: u64,
    pub send_flags: ::std::os::raw::c_uint,
    pub bind_info: ibv_mw_bind_info,
}

#[repr(C)]
pub struct ibv_mr {
    pub context: *mut ibv_context,
    pub pd: *mut ibv_pd,
    pub addr: *mut c_void,
    pub length: usize,
    pub handle: u32,
    pub lkey: u32,
    pub rkey: u32,
}

pub type __u32 = ::std::os::raw::c_uint;
pub type __be32 = __u32; //big endien, used in network protocols

#[repr(C)]
pub union imm_data_invalidated_rkey_union_t {
    pub imm_data: __be32,
    pub invalidated_rkey: u32,
}

pub mod ibv_wc_status {
    pub type Type = ::std::os::raw::c_uint;
    pub const IBV_WC_SUCCESS: Type = 0;
    pub const IBV_WC_LOC_LEN_ERR: Type = 1;
    pub const IBV_WC_LOC_QP_OP_ERR: Type = 2;
    pub const IBV_WC_LOC_EEC_OP_ERR: Type = 3;
    pub const IBV_WC_LOC_PROT_ERR: Type = 4;
    pub const IBV_WC_WR_FLUSH_ERR: Type = 5;
    pub const IBV_WC_MW_BIND_ERR: Type = 6;
    pub const IBV_WC_BAD_RESP_ERR: Type = 7;
    pub const IBV_WC_LOC_ACCESS_ERR: Type = 8;
    pub const IBV_WC_REM_INV_REQ_ERR: Type = 9;
    pub const IBV_WC_REM_ACCESS_ERR: Type = 10;
    pub const IBV_WC_REM_OP_ERR: Type = 11;
    pub const IBV_WC_RETRY_EXC_ERR: Type = 12;
    pub const IBV_WC_RNR_RETRY_EXC_ERR: Type = 13;
    pub const IBV_WC_LOC_RDD_VIOL_ERR: Type = 14;
    pub const IBV_WC_REM_INV_RD_REQ_ERR: Type = 15;
    pub const IBV_WC_REM_ABORT_ERR: Type = 16;
    pub const IBV_WC_INV_EECN_ERR: Type = 17;
    pub const IBV_WC_INV_EEC_STATE_ERR: Type = 18;
    pub const IBV_WC_FATAL_ERR: Type = 19;
    pub const IBV_WC_RESP_TIMEOUT_ERR: Type = 20;
    pub const IBV_WC_GENERAL_ERR: Type = 21;
    pub const IBV_WC_TM_ERR: Type = 22;
    pub const IBV_WC_TM_RNDV_INCOMPLETE: Type = 23;
}

pub mod ibv_wc_opcode {
    pub type Type = ::std::os::raw::c_uint;
    pub const IBV_WC_SEND: Type = 0;
    pub const IBV_WC_RDMA_WRITE: Type = 1;
    pub const IBV_WC_RDMA_READ: Type = 2;
    pub const IBV_WC_COMP_SWAP: Type = 3;
    pub const IBV_WC_FETCH_ADD: Type = 4;
    pub const IBV_WC_BIND_MW: Type = 5;
    pub const IBV_WC_LOCAL_INV: Type = 6;
    pub const IBV_WC_TSO: Type = 7;
    pub const IBV_WC_RECV: Type = 128;
    pub const IBV_WC_RECV_RDMA_WITH_IMM: Type = 129;
    pub const IBV_WC_TM_ADD: Type = 130;
    pub const IBV_WC_TM_DEL: Type = 131;
    pub const IBV_WC_TM_SYNC: Type = 132;
    pub const IBV_WC_TM_RECV: Type = 133;
    pub const IBV_WC_TM_NO_TAG: Type = 134;
    pub const IBV_WC_DRIVER1: Type = 135;
}

#[repr(C)]
pub struct ibv_wc {
    pub wr_id: u64,
    pub status: ibv_wc_status::Type,
    pub opcode: ibv_wc_opcode::Type,
    pub vendor_err: u32,
    pub byte_len: u32,
    pub imm_data_invalidated_rkey_union: imm_data_invalidated_rkey_union_t,
    pub qp_num: u32,
    pub src_qp: u32,
    pub wc_flags: ::std::os::raw::c_uint,
    pub pkey_index: u16,
    pub slid: u16,
    pub sl: u8,
    pub dlid_path_bits: u8,
}

#[repr(C)]
pub struct ibv_sge {
    pub addr: u64,
    pub length: u32,
    pub lkey: u32,
}

#[repr(C)]
pub struct ibv_recv_wr {
    pub wr_id: u64,
    pub next: *mut ibv_recv_wr,
    pub sg_list: *mut ibv_sge,
    pub num_sge: ::std::os::raw::c_int,
}

pub mod ibv_qp_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const IBV_QPT_RC: Type = 2;
    pub const IBV_QPT_UC: Type = 3;
    pub const IBV_QPT_UD: Type = 4;
    pub const IBV_QPT_RAW_PACKET: Type = 8;
    pub const IBV_QPT_XRC_SEND: Type = 9;
    pub const IBV_QPT_XRC_RECV: Type = 10;
    pub const IBV_QPT_DRIVER: Type = 255;
}

pub mod ibv_wr_opcode {
    pub type Type = ::std::os::raw::c_uint;
    pub const IBV_WR_RDMA_WRITE: Type = 0;
    pub const IBV_WR_RDMA_WRITE_WITH_IMM: Type = 1;
    pub const IBV_WR_SEND: Type = 2;
    pub const IBV_WR_SEND_WITH_IMM: Type = 3;
    pub const IBV_WR_RDMA_READ: Type = 4;
    pub const IBV_WR_ATOMIC_CMP_AND_SWP: Type = 5;
    pub const IBV_WR_ATOMIC_FETCH_AND_ADD: Type = 6;
    pub const IBV_WR_LOCAL_INV: Type = 7;
    pub const IBV_WR_BIND_MW: Type = 8;
    pub const IBV_WR_SEND_WITH_INV: Type = 9;
    pub const IBV_WR_TSO: Type = 10;
    pub const IBV_WR_DRIVER1: Type = 11;
}


#[repr(C)]
pub struct ibv_ah {
    pub context: *mut ibv_context,
    pub pd: *mut ibv_pd,
    pub handle: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct rdma_t {
    pub remote_addr: u64,
    pub rkey: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct atomic_t {
    pub remote_addr: u64,
    pub compare_add: u64,
    pub swap: u64,
    pub rkey: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ud_t {
    pub ah: *mut ibv_ah,
    pub remote_qpn: u32,
    pub remote_qkey: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union wr_union_t {
    pub rdma: rdma_t, 
    pub atomic: atomic_t,
    pub ud: ud_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct xrc_t {
    pub remote_srqn: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union qp_type_union_t {
    pub xrc: xrc_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct bind_mw_t {
    pub mw: *mut ibv_mw,
    pub rkey: u32,
    pub bind_info: ibv_mw_bind_info,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct tso_t {
    pub hdr: *mut c_void,
    pub hdr_sz: u16,
    pub mss: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union tso_union_t {
    pub bind_mw: bind_mw_t,
    pub tso: tso_t,
}

#[repr(C)]
pub struct ibv_send_wr {
    pub wr_id: u64,
    pub next: *mut ibv_send_wr,
    pub sg_list: *mut ibv_sge,
    pub num_sge: ::std::os::raw::c_int,
    pub opcode: ibv_wr_opcode::Type,
    pub send_flags: ::std::os::raw::c_uint,
    pub imm_data_invalidated_rkey_union: imm_data_invalidated_rkey_union_t,
    pub wr_union: wr_union_t,
    pub qp_type_union: qp_type_union_t,
    pub tso_union: tso_union_t,
}

#[repr(C)]
pub struct ibv_context_ops {
    pub _compat_query_device: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_query_port: Option<unsafe extern "C" fn(context: *mut ibv_context, port_num: u8, port_attr: *mut _compat_ibv_port_attr,) -> c_int,>,
    pub _compat_alloc_pd: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_dealloc_pd: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_reg_mr: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_rereg_mr: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_dereg_mr: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub alloc_mw: Option<unsafe extern "C" fn(pd: *mut ibv_pd, type_: ibv_mw_type::Type) -> *mut ibv_mw,>,
    pub bind_mw: Option<unsafe extern "C" fn(qp: *mut ibv_qp, mw: *mut ibv_mw, mw_bind: *mut ibv_mw_bind,) -> c_int,>,
    pub dealloc_mw: Option<unsafe extern "C" fn(mw: *mut ibv_mw) -> c_int>,
    pub _compat_create_cq: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub poll_cq: Option<unsafe extern "C" fn( cq: *mut ibv_cq, num_entries: c_int, wc: *mut ibv_wc) -> c_int,>,
    pub req_notify_cq: Option<unsafe extern "C" fn(cq: *mut ibv_cq,solicited_only: c_int,) -> c_int,>,
    pub _compat_cq_event: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_resize_cq: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_destroy_cq: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_create_srq: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_modify_srq: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_query_srq: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_destroy_srq: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub post_srq_recv: Option<unsafe extern "C" fn(srq: *mut ibv_srq, recv_wr: *mut ibv_recv_wr, bad_recv_wr: *mut *mut ibv_recv_wr,) -> c_int,>,
    pub _compat_create_qp: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_query_qp: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_modify_qp: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_destroy_qp: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub post_send: Option<unsafe extern "C" fn( qp: *mut ibv_qp, wr: *mut ibv_send_wr, bad_wr: *mut *mut ibv_send_wr,) -> c_int,>,
    pub post_recv: Option<unsafe extern "C" fn( qp: *mut ibv_qp, wr: *mut ibv_recv_wr, bad_wr: *mut *mut ibv_recv_wr,) -> c_int,>,
    pub _compat_create_ah: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_destroy_ah: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_attach_mcast: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_detach_mcast: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub _compat_async_event: Option<unsafe extern "C" fn() -> *mut c_void>,
}

#[repr(C)]
pub struct ibv_context {
    pub device: *mut ibv_device,
    pub ops: ibv_context_ops,
    pub cmd_fd: c_int,
    pub async_fd: c_int,
    pub num_comp_vectors: c_int,
    pub mutex: libc::pthread_mutex_t,
    pub abi_compat: *mut c_void,
}

#[repr(C)]
pub struct ibv_pd {
    pub context: *mut ibv_context,
    pub handle: u32,
}

#[repr(C)]
pub struct ibv_qp_cap {
    pub max_send_wr:     u32,
    pub max_recv_wr:     u32,   
    pub max_send_sge:    u32,
    pub max_recv_sge:    u32,  
    pub max_inline_data: u32,
}

#[repr(C)]
pub struct ibv_comp_channel {
    context: *mut ibv_context,
    fd: c_int,
    refcnt: c_int,
}

#[repr(C)]
pub struct ibv_cq {
    pub context: *mut ibv_context,
    pub channel: *mut ibv_comp_channel,
    pub cq_context: *mut c_void,
    pub handle: u32,
    pub cqe: c_int,
    pub mutex: libc::pthread_mutex_t,
    pub cond: libc::pthread_cond_t,
    pub comp_events_completed: u32,
    pub async_events_completed: u32,
}

#[repr(C)]
pub struct ibv_srq {
    pub context:      *mut ibv_context,
    pub srq_context:  *mut c_void,
    pub pd:           *mut ibv_pd,
    pub handle:            u32,
    pub mutex: libc::pthread_mutex_t,
    pub cond: libc::pthread_cond_t,
    pub events_completed:  u32,
}

pub mod ibv_qp_state {
    pub type Type = ::std::os::raw::c_uint;
    pub const IBV_QPS_RESET: Type = 0;
    pub const IBV_QPS_INIT: Type = 1;
    pub const IBV_QPS_RTR: Type = 2;
    pub const IBV_QPS_RTS: Type = 3;
    pub const IBV_QPS_SQD: Type = 4;
    pub const IBV_QPS_SQE: Type = 5;
    pub const IBV_QPS_ERR: Type = 6;
    pub const IBV_QPS_UNKNOWN: Type = 7;
}

#[repr(C)]
pub struct ibv_qp {
    pub context: *mut ibv_context,
    pub qp_context: *mut c_void,
    pub pd: *mut ibv_pd,
    pub send_cq: *mut ibv_cq,
    pub recv_cq: *mut ibv_cq,
    pub srq: *mut ibv_srq,
    pub handle: u32,
    pub qp_num: u32,
    pub state: ibv_qp_state::Type,
    pub qp_type: ibv_qp_type::Type,
    pub mutex: libc::pthread_mutex_t,
    pub cond: libc::pthread_cond_t,
    pub events_completed: u32,
}

#[repr(C)]
pub struct ibv_qp_init_attr {
    pub qp_context: *mut c_void,
    pub send_cq: *mut ibv_cq,
    pub recv_cq: *mut ibv_cq,
    pub srq: *mut ibv_srq,
    pub cap: ibv_qp_cap,
    pub qp_type: ibv_qp_type::Type, //ibv_qp_type is an enum 
    pub sq_sig_all: c_int,
}

extern "C" {
    pub fn ibv_create_qp(pd: *mut ibv_pd, qp_init_attr: *mut ibv_qp_init_attr) -> *mut ibv_qp;
}

fn main() {
    
}
