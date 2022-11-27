

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// pub fn init() {

// unsafe { initVc() };

// }
extern crate libloading as lib;

use std::collections::HashMap;
use std::thread;
use std::ffi::{CString, CStr};
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use ethereum_types::{Address, BigEndianHash, H160, H256, U256};
use std::str::FromStr;
use keccak_hash::keccak;
use csv::Writer;
//state info
static mut BALANCE_STATE_SIZE: u64 = 0u64;
static mut TRIE_STATE_SIZE: u64 = 0u64;
static mut revert_output_vec: Vec<(SystemTime,u64, u64, u64, u64)> = Vec::new();
static mut EFFECTIVE_BLOCK_NUMBER: u64 = 0u64;
static mut CONFIRMED_BLOCK_NUMBER:u64 = 0u64;
static mut JUNK_BLOCK_NUMBER:u64 = 0u64;
static mut LAST_CHECKPOINT_NUMBER: u64 = 0u64;
// reverts the state if the flag is found to be true.
static mut STATE_REVERT: bool = false;
//no transaction is mined; hence empty blocks.
static mut STATE_DORMANT: bool = false;
// whether hyperproofs are integrated or not
static mut IS_AGG: bool = true;
static mut AGG_STARTED: bool = false;
static mut SHARD: u64 = 0u64;
static mut ID: u64 = 0u64;
static mut LASTCOMMITROUND: u64 = 999u64;
static mut GENESISCOMMIT: u64 = 0u64;
static mut LATESTIMPORTEDBLOCK: u64 = 0u64;
static mut SLOADCOUNT: u64 = 0u64;
static mut SSTORECOUNT: u64 = 0u64;
static mut BALREADCOUNT: u64 = 0u64;
static mut BALWRITECOUNT: u64 = 0u64;
static mut TOTALGAS: f32 = 0f32;
static mut TOTALSIZE: f32 = 0f32;
static mut TOTALACCOUNTPROOF: u64 = 0u64;
static mut TOTALKEYPROOF: u64 = 0u64;
static mut TOTALACCOUNTNUM: u64 = 0u64;
static mut TOTALKEYNUM: u64 = 0u64;

static mut HOPCOUNT_1: u64 = 0u64;
static mut HOPCOUNT_2: u64 = 0u64;
static mut HOPCOUNT_3: u64 = 0u64;
static mut HOPCOUNT_4: u64 = 0u64;
static mut HOPCOUNT_5: u64 = 0u64;
static mut HOPCOUNT_6: u64 = 0u64;
static mut HOPCOUNT_7: u64 = 0u64;
static mut REVERTED: u64 = 0u64;
//vec for the verification data
static mut VERIFICATION_DATA: Vec<Vec<(H160, U256)>> = Vec::new();
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AggProof{
    pub proof: String,
    pub ready: bool,
    pub address: Vec<Address>,
    pub balance: Vec<U256>,
}
impl AggProof{
    // pub fn write(){
    //
    // }
    pub fn push_verification_data(data: Vec<(H160, U256)>){
        unsafe {
            VERIFICATION_DATA.push(data);
        }
    }
    pub fn pop_verification_data()-> Vec<(H160,U256)>{
        unsafe {
            assert!(VERIFICATION_DATA.len()>0);
            let v = VERIFICATION_DATA[0].clone();
            VERIFICATION_DATA.remove(0);
            v
        }
    }
    pub fn incr_hop_count(hop:u64){
        match hop {
            x if x==1u64  => unsafe{HOPCOUNT_1 += 1u64;},
            x if x==2u64  => unsafe{HOPCOUNT_2 += 1u64;},
            x if x==3u64  => unsafe{HOPCOUNT_3 += 1u64;},
            x if x==4u64  => unsafe{HOPCOUNT_4 += 1u64;},
            x if x==5u64  => unsafe{HOPCOUNT_5 += 1u64;},
            x if x==6u64  => unsafe{HOPCOUNT_6 += 1u64;},
            _  => unsafe{HOPCOUNT_7 += 1u64;},
        }
    }
    pub fn incr_balance_state_size(b:u64){
        unsafe{BALANCE_STATE_SIZE += b;}
    }
    pub fn get_balance_state_size()-> u64{
        unsafe{
            let o = BALANCE_STATE_SIZE;
            o
        }
    }
    pub fn incr_trie_state_size(b:u64){
        unsafe{TRIE_STATE_SIZE += b;}
    }
    pub fn get_trie_state_size()-> u64{
        unsafe{
            let o = TRIE_STATE_SIZE;
            o
        }
    }
    pub fn push_revert_vec(b:(SystemTime, u64, u64, u64, u64)){
        unsafe{revert_output_vec.push(b);}
    }
    pub fn get_revert_vec()-> Vec<(SystemTime, u64, u64, u64, u64)>{
        unsafe {
            let o = revert_output_vec.clone();
            o
        }
    }
    pub fn incr_junk_block(b:u64){
        unsafe{JUNK_BLOCK_NUMBER += b;}
    }

    pub fn get_junk_block()-> u64{
        unsafe {
            let o = JUNK_BLOCK_NUMBER;
            o
        }
    }
    pub fn incr_checkpoint_number(b:u64){
        unsafe{LAST_CHECKPOINT_NUMBER += b;}
    }

    pub fn get_checkpoint_number()-> u64{
        unsafe {
            let o = LAST_CHECKPOINT_NUMBER;
            o
        }
    }
    pub fn incr_confirmed_block(b:u64){
        unsafe{CONFIRMED_BLOCK_NUMBER += b;}
    }

    pub fn get_confirmed_block()-> u64{
        unsafe {
            let o = CONFIRMED_BLOCK_NUMBER;
            o
        }
    }
    pub fn incr_effective_block(b:u64){
        unsafe{EFFECTIVE_BLOCK_NUMBER += b;}
    }
    pub fn decr_effective_block(b:u64){
        unsafe{EFFECTIVE_BLOCK_NUMBER = EFFECTIVE_BLOCK_NUMBER.saturating_sub( b);}
    }
    pub fn get_effective_block()-> u64{
        unsafe {
            let o = EFFECTIVE_BLOCK_NUMBER;
            o
        }
    }
    pub fn incr_reverted_count(){
            unsafe{REVERTED += 1u64;}
    }

    pub fn incr_total_gas(gas:f32){
        unsafe{TOTALGAS += gas;}
    }

    pub fn get_total_gas()-> f32{
        unsafe {
            let o = TOTALGAS;
            o
        }
    }

    pub fn incr_total_size(size:f32){
        unsafe{TOTALSIZE += size;}
    }

    pub fn get_total_size()-> f32{
        unsafe {
            let o = TOTALSIZE;
            o
        }
    }
    pub fn update_state_revert(update:bool){
        unsafe{STATE_REVERT = update;}
    }
    pub fn get_state_revert()-> bool{
        unsafe {
            let o = STATE_REVERT;
            o
        }
    }
    pub fn update_state_dormant(update:bool){
        unsafe{STATE_DORMANT = update;}
    }
    pub fn get_state_dormant()-> bool{
        unsafe {
            let o = STATE_DORMANT;
            o
        }
    }
    pub fn incr_total_account_proof(len:u64){
        unsafe{TOTALACCOUNTPROOF += len;}
    }

    pub fn get_total_account_proof()-> u64{
        unsafe {
            let o = TOTALACCOUNTPROOF;
            o
        }
    }

    pub fn incr_total_key_proof(len:u64){
        unsafe{TOTALKEYPROOF += len;}
    }

    pub fn get_total_key_proof()-> u64{
        unsafe {
            let o = TOTALKEYPROOF;
            o
        }
    }
    pub fn incr_total_key_num(len:u64){
        unsafe{TOTALKEYNUM += len;}
    }

    pub fn get_total_key_num()-> u64{
        unsafe {
            let o = TOTALKEYNUM;
            o
        }
    }
    pub fn incr_total_account_num(len:u64){
        unsafe{TOTALACCOUNTNUM += len;}
    }

    pub fn get_total_account_num()-> u64{
        unsafe {
            let o = TOTALACCOUNTNUM;
            o
        }
    }
    pub fn get_reverted_count()-> u64{
        unsafe {
            let o = REVERTED;
            o
        }
    }
    pub fn agg_started()-> bool{
        unsafe {
            let o = AGG_STARTED;
            o
        }
    }
    pub fn is_agg()-> bool{
        unsafe {
            let o = IS_AGG;
            o
        }
    }
    pub fn set_agg_started(){
        unsafe{AGG_STARTED = true;}
    }
    pub fn get_hop_count(hop:u64)->u64{
        match hop {
            x if x==1u64  => unsafe{let o = HOPCOUNT_1;
                o},
            x if x==2u64  => unsafe{let o = HOPCOUNT_2;
                o},
            x if x==3u64  => unsafe{let o = HOPCOUNT_3;
                o},
            x if x==4u64  => unsafe{let o = HOPCOUNT_4;
                o},
            x if x==5u64  => unsafe{let o = HOPCOUNT_5;
                o},
            x if x==6u64  => unsafe{let o = HOPCOUNT_6;
                o},
            _ => unsafe{let o = HOPCOUNT_7;
                o},
        }
    }
    pub fn get_latest_imported_block()->u64{
        unsafe {
            let o = LATESTIMPORTEDBLOCK;
            o }
    }
    pub fn set_latest_imported_block(b: u64){
        unsafe {
             LATESTIMPORTEDBLOCK = b;
             }
    }
    pub fn get_sload_count()->u64{
        unsafe {
            let o = SLOADCOUNT;
            o }
    }
    pub fn incr_sload_count(delta: u64) {
        unsafe{
            let mut o = SLOADCOUNT;
            o+= delta;
            SLOADCOUNT = o;
        }
    }
    pub fn get_sstore_count()->u64{
        unsafe {
            let o = SSTORECOUNT;
            o }
    }
    pub fn incr_sstore_count(delta: u64) {
        unsafe{
            let mut o = SSTORECOUNT;
            o+= delta;
            SSTORECOUNT = o;
        }
    }
    pub fn get_bal_read_count()->u64{
        unsafe {
            let o = BALREADCOUNT;
            o }
    }
    pub fn incr_bal_read_count(delta: u64) {
        unsafe{
            let mut o = BALREADCOUNT;
            o+= delta;
            BALREADCOUNT = o;
        }
    }
    pub fn get_bal_write_count()->u64{
        unsafe {
            let o = BALWRITECOUNT;
            o }
    }
    pub fn incr_bal_write_count(delta: u64) {
        unsafe{
            let mut o = BALWRITECOUNT;
            o+= delta;
            BALWRITECOUNT = o;
        }
    }
    pub fn new() -> Self {
        AggProof{
            proof: String::new(),
            ready:false,
            address: Vec::new(),
            balance: Vec::new(),
        }
    }
    pub fn concat_hash(x: H160, y: H256) -> H160{
        let l = keccak([x.as_bytes(), y.as_bytes()].concat());
        H160::from(l)
    }
    pub fn create_proof(&mut self) -> (){
        if self.address.len() == 0{
            return
        }
        self.ready = false;
        for i in 0..self.address.len(){
            pushAddressCommit(self.address[i].to_low_u64_be().rem_euclid(2u64.pow(AggProof::hyperproof_bits())),0u64);
        }
        match agg(0u64) {
           Ok(T) => {
               self.proof = T.0;
               self.ready = true;
           },
            _ => {},
        }

    }
    pub fn set_author_shard(address: Address) -> u64 {
        let _s1 = Address::from_str("00bd138abd70e2f00903268f3db08f2d25677c9e").unwrap();
        let _s2 = Address::from_str("00aa39d30f0d20ff03a22ccfc30b7efbfca597c2").unwrap();
        let _s3 = Address::from_str("002e28950558fbede1a9675cb113f0bd20912019").unwrap();
        let _s4 = Address::from_str("00a94ac799442fb13de8302026fd03068ba6a428").unwrap();
        let _s5 = Address::from_str("00d4f0e12020c15487b2a525abcb27de647c12de").unwrap();
        let _s6 = Address::from_str("001f477a48a01d2561e324f874782b2dd8167772").unwrap();
        let _s7 = Address::from_str("006137d98307ab6691ccedb7a10b295da8ae1035").unwrap();
        let _s8 = Address::from_str("003f3b1f635b2dd9a4518c33098e5f72214d6a1e").unwrap();
        let _s9 = Address::from_str("008272a8cfd2d3d0f3edc823b1bb729cb73f09db").unwrap();
        let _s10 = Address::from_str("001ce0f63558e2fe10806d132d64d2b2f63ef64e").unwrap();
        let _s11 = Address::from_str("0038658156bcb555c1aa24d1adabb57c36fbcd6d").unwrap();
        let _s12 = Address::from_str("006a8e26c9653d22f1cadb22a81428deaa8554be").unwrap();
        let _s13 = Address::from_str("00c3ca2fd819f4d2ea30c9fd99bf80c7c86f1f25").unwrap();
        let _s14 = Address::from_str("00734b960d1edd54e50192e47acfdc8af0fbbd20").unwrap();
        let _s15 = Address::from_str("002db24c08ed9397bc77a554e55f80d56be7b15f").unwrap();
        let _s16 = Address::from_str("004f49d9267bce6bdefc0fe9065269fa5d24ead9").unwrap();
        match address {
            x if x==_s1  => unsafe{SHARD = 0u64;
            0u64},
            x if x==_s2 => unsafe{SHARD = 1u64;
                1u64},
            x if x==_s3 => unsafe{SHARD = 2u64;
                2u64},
            x if x==_s4 => unsafe{SHARD = 3u64;
                3u64},
            x if x==_s5 => unsafe{SHARD = 0u64;
                0u64},
            x if x==_s6 => unsafe{SHARD = 1u64;
                1u64},
            x if x==_s7 => unsafe{SHARD = 2u64;
                2u64},
            x if x==_s8 => unsafe{SHARD = 3u64;
                3u64},
            x if x==_s9 => unsafe{SHARD = 0u64;
                0u64},
            x if x==_s10 => unsafe{SHARD = 1u64;
                1u64},
            x if x==_s11 => unsafe{SHARD = 2u64;
                2u64},
            x if x==_s12 => unsafe{SHARD = 3u64;
                3u64},
            x if x==_s13 => unsafe{SHARD = 0u64;
                0u64},
            x if x==_s14 => unsafe{SHARD = 1u64;
                1u64},
            x if x==_s15 => unsafe{SHARD = 2u64;
                2u64},
            x if x==_s16 => unsafe{SHARD = 3u64;
                3u64},
            _ => unsafe{SHARD = 999u64; println!("panic!, shard not recognised");
                999u64},
        }
    }

    pub fn set_author_id(address: Address) -> u64 {
        let _s1 = Address::from_str("00bd138abd70e2f00903268f3db08f2d25677c9e").unwrap();
        let _s2 = Address::from_str("00aa39d30f0d20ff03a22ccfc30b7efbfca597c2").unwrap();
        let _s3 = Address::from_str("002e28950558fbede1a9675cb113f0bd20912019").unwrap();
        let _s4 = Address::from_str("00a94ac799442fb13de8302026fd03068ba6a428").unwrap();
        let _s5 = Address::from_str("00d4f0e12020c15487b2a525abcb27de647c12de").unwrap();
        let _s6 = Address::from_str("001f477a48a01d2561e324f874782b2dd8167772").unwrap();
        let _s7 = Address::from_str("006137d98307ab6691ccedb7a10b295da8ae1035").unwrap();
        let _s8 = Address::from_str("003f3b1f635b2dd9a4518c33098e5f72214d6a1e").unwrap();
        let _s9 = Address::from_str("008272a8cfd2d3d0f3edc823b1bb729cb73f09db").unwrap();
        let _s10 = Address::from_str("001ce0f63558e2fe10806d132d64d2b2f63ef64e").unwrap();
        let _s11 = Address::from_str("0038658156bcb555c1aa24d1adabb57c36fbcd6d").unwrap();
        let _s12 = Address::from_str("006a8e26c9653d22f1cadb22a81428deaa8554be").unwrap();
        let _s13 = Address::from_str("00c3ca2fd819f4d2ea30c9fd99bf80c7c86f1f25").unwrap();
        let _s14 = Address::from_str("00734b960d1edd54e50192e47acfdc8af0fbbd20").unwrap();
        let _s15 = Address::from_str("002db24c08ed9397bc77a554e55f80d56be7b15f").unwrap();
        let _s16 = Address::from_str("004f49d9267bce6bdefc0fe9065269fa5d24ead9").unwrap();
        match address {
            x if x==_s1  => unsafe{ID = 0u64;
                0u64},
            x if x==_s2 => unsafe{ID = 1u64;
                1u64},
            x if x==_s3 => unsafe{ID = 2u64;
                2u64},
            x if x==_s4 => unsafe{ID = 3u64;
                3u64},
            x if x==_s5 => unsafe{ID = 4u64;
                4u64},
            x if x==_s6 => unsafe{ID = 5u64;
                5u64},
            x if x==_s7 => unsafe{ID = 6u64;
                6u64},
            x if x==_s8 => unsafe{ID = 7u64;
                7u64},
            x if x==_s9 => unsafe{ID = 8u64;
                8u64},
            x if x==_s10 => unsafe{ID = 9u64;
                9u64},
            x if x==_s11 => unsafe{ID = 10u64;
                10u64},
            x if x==_s12 => unsafe{ID = 11u64;
                11u64},
            x if x==_s13 => unsafe{ID = 12u64;
                12u64},
            x if x==_s14 => unsafe{ID = 13u64;
                13u64},
            x if x==_s15 => unsafe{ID = 14u64;
                14u64},
            x if x==_s16 => unsafe{ID = 15u64;
                15u64},
            _ => unsafe{ID = 999u64; println!("panic!, id not recognised");
                999u64},
        }
    }
    pub fn get_id() -> u64 {
        unsafe {
            let o = ID;
            o }
    }
    pub fn author_id(address: Address) -> u64 {
        let _s1 = Address::from_str("00bd138abd70e2f00903268f3db08f2d25677c9e").unwrap();
        let _s2 = Address::from_str("00aa39d30f0d20ff03a22ccfc30b7efbfca597c2").unwrap();
        let _s3 = Address::from_str("002e28950558fbede1a9675cb113f0bd20912019").unwrap();
        let _s4 = Address::from_str("00a94ac799442fb13de8302026fd03068ba6a428").unwrap();
        let _s5 = Address::from_str("00d4f0e12020c15487b2a525abcb27de647c12de").unwrap();
        let _s6 = Address::from_str("001f477a48a01d2561e324f874782b2dd8167772").unwrap();
        let _s7 = Address::from_str("006137d98307ab6691ccedb7a10b295da8ae1035").unwrap();
        let _s8 = Address::from_str("003f3b1f635b2dd9a4518c33098e5f72214d6a1e").unwrap();
        let _s9 = Address::from_str("008272a8cfd2d3d0f3edc823b1bb729cb73f09db").unwrap();
        let _s10 = Address::from_str("001ce0f63558e2fe10806d132d64d2b2f63ef64e").unwrap();
        let _s11 = Address::from_str("0038658156bcb555c1aa24d1adabb57c36fbcd6d").unwrap();
        let _s12 = Address::from_str("006a8e26c9653d22f1cadb22a81428deaa8554be").unwrap();
        let _s13 = Address::from_str("00c3ca2fd819f4d2ea30c9fd99bf80c7c86f1f25").unwrap();
        let _s14 = Address::from_str("00734b960d1edd54e50192e47acfdc8af0fbbd20").unwrap();
        let _s15 = Address::from_str("002db24c08ed9397bc77a554e55f80d56be7b15f").unwrap();
        let _s16 = Address::from_str("004f49d9267bce6bdefc0fe9065269fa5d24ead9").unwrap();
        match address {
           x if x==_s1  => 0u64,
            x if x==_s2 => 1u64,
            x if x==_s3 => 2u64,
            x if x==_s4 => 3u64,
            x if x==_s5 => 4u64,
            x if x==_s6 => 5u64,
            x if x==_s7 => 6u64,
            x if x==_s8 => 7u64,
            x if x==_s9 => 8u64,
            x if x==_s10 => 9u64,
            x if x==_s11 => 10u64,
            x if x==_s12 => 11u64,
            x if x==_s13 => 12u64,
            x if x==_s14 => 13u64,
            x if x==_s15 => 14u64,
            x if x==_s16 => 15u64,
            _ => 999u64,
        }

    }
    pub fn get_shard() -> u64 {
        unsafe {
            let o = SHARD;
        o }
    }
    pub fn set_genesis_commit(status: u64) { unsafe{GENESISCOMMIT = status; } }
    pub fn get_genesis_commit() -> u64 {
        unsafe {
            let o = GENESISCOMMIT;
            o }
    }
    pub fn set_last_commit_shard(round: u64){
        unsafe{LASTCOMMITROUND = round; }
    }

    pub fn get_last_commit_round() -> u64 {
        unsafe {
            let o = LASTCOMMITROUND;
            o }
    }

    pub fn shard_count() -> u64 {
        4u64
    }
    pub fn node_count() -> u64 {
        16u64
    }
    pub fn txn_lifetime() -> u64 {
        20u64
    }
    pub fn incomplete_txn_buffer_length() -> u64{16000u64}
    pub fn hyperproof_bits()-> u32{24u32}
    pub fn block_data_count() -> u64 {2048u64}
    // pub fn author_shard(address: Address) -> u64 {
    //     let _s1 = Address::from_str("00bd138abd70e2f00903268f3db08f2d25677c9e").unwrap();
    //     let _s2 = Address::from_str("00aa39d30f0d20ff03a22ccfc30b7efbfca597c2").unwrap();
    //     let _s3 = Address::from_str("002e28950558fbede1a9675cb113f0bd20912019").unwrap();
    //     let _s4 = Address::from_str("00a94ac799442fb13de8302026fd03068ba6a428").unwrap();
    //     match address {
    //        x if x==_s1  => 0u64,
    //         x if x==_s2 => 1u64,
    //         x if x==_s3 => 0u64,
    //         x if x==_s4 => 1u64,
    //         _ => 999u64,
    //     }
    //
    // }
    pub fn init(round:u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(r:u64) -> i64 > = lib.get(b"initVc")?;
            Ok(func(round))
        }
    }
    pub fn agg(nativeShard: u64) -> lib::Result<(String,bool)>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> aggVc_return > = lib.get(b"aggVc")?;
            match func(nativeShard) {
                output => match output.r1 {
                    1u8 => Ok((CStr::from_ptr(output.r0)
                                   .to_string_lossy()
                                   .into_owned(), true)
                    ),
                    _ => Ok((CStr::from_ptr(output.r0)
                                 .to_string_lossy()
                                 .into_owned(), false)
                    ),
                }
            }
        }
    }

    pub fn pushAddressDelta(address: u64, delta: String, shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        let c_delta = CString::new(delta)?;
        let go_str_delta = GoString {
            p: c_delta.as_ptr(),
            n: c_delta.as_bytes().len() as isize,
        };
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(a: u64, d: GoString, s: u64) -> i64 > = lib.get(b"pushAddressDeltaVc")?;
            Ok(func(address,go_str_delta,shard))
        }
    }
    pub fn resetAddressDelta(shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> i64 > = lib.get(b"resetAddressDeltaVc")?;
            Ok(func(shard))
        }
    }
    //push address for which proof needs to be aggregated
    pub fn pushAddressCommit(address: u64, shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(a: u64, s: u64) -> i64 > = lib.get(b"pushAddressCommitVc")?;
            Ok(func(address,shard))
        }
    }
    //push address for which proof needs to be aggregated
    pub fn resetAddressCommit(shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> i64 > = lib.get(b"resetAddressCommitVc")?;
            Ok(func(shard))
        }
    }

    pub fn pushAddressBalanceVerify(address: u64, bal: String, shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        let c_bal = CString::new(bal)?;
        let go_str_bal = GoString {
            p: c_bal.as_ptr(),
            n: c_bal.as_bytes().len() as isize,
        };
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(a: u64, b: GoString, s: u64) -> i64 > = lib.get(b"pushAddressBalanceVerifyVc")?;
            Ok(func(address,go_str_bal,shard))
        }
    }
    pub fn resetAddressBalanceVerify(shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> i64 > = lib.get(b"resetAddressBalanceVerifyVc")?;
            Ok(func(shard))
        }
    }
    pub fn verifyProof(input: String, shard:u64,round:u64) -> lib::Result<bool>{
        let c_input = CString::new(input)?;
        let go_str_input = GoString {
            p: c_input.as_ptr(),
            n: c_input.as_bytes().len() as isize,
        };
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(i: GoString, s:u64, r:u64) -> u8> = lib.get(b"verifyProofVc")?;
            match func(go_str_input, shard, round) {
                1u8 => Ok(true),
                _ => Ok(false)
            }
        }
    }
    pub fn commit(nativeShard: u64, round: u64, revert: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(n: u64, r: u64, rv: u64) -> i64 > = lib.get(b"commitVc")?;
            Ok(func(nativeShard,round,revert))
        }
    }
    pub fn updateTree(nativeShard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(n: u64) -> i64 > = lib.get(b"updateShardProofTreeVc")?;
            Ok(func(nativeShard))
        }
    }
    pub fn resetPrevCommit() -> lib::Result<i64>{
        let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn() -> i64 > = lib.get(b"prevDigestResetVc")?;
            Ok(func())
        }
    }
}
pub fn init(round:u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(r:u64) -> i64 > = lib.get(b"initVc")?;
        Ok(func(round))
    }
}

pub fn pushAddressDelta(address: u64, delta: String, shard: u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
    let c_delta = CString::new(delta)?;
    let go_str_delta = GoString {
      p: c_delta.as_ptr(),
      n: c_delta.as_bytes().len() as isize,
    };
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(a: u64, d: GoString, s: u64) -> i64 > = lib.get(b"pushAddressDeltaVc")?;
        Ok(func(address,go_str_delta,shard))
    }
}
//push address for which proof needs to be aggregated
pub fn pushAddressCommit(address: u64, shard: u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(a: u64, s: u64) -> i64 > = lib.get(b"pushAddressCommitVc")?;
        Ok(func(address,shard))
    }
}

pub fn pushAddressBalanceVerify(address: u64, bal: String, shard: u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
    let c_bal = CString::new(bal)?;
    let go_str_bal = GoString {
      p: c_bal.as_ptr(),
      n: c_bal.as_bytes().len() as isize,
    };
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(a: u64, b: GoString, s: u64) -> i64 > = lib.get(b"pushAddressBalanceVerifyVc")?;
        Ok(func(address,go_str_bal,shard))
    }
}

pub fn agg(nativeShard: u64) -> lib::Result<(String,bool)>{
    let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> aggVc_return > = lib.get(b"aggVc")?;
        match func(nativeShard) {
            output => match output.r1 {
                1u8 => Ok((CStr::from_ptr(output.r0)
                        .to_string_lossy()
                        .into_owned(), true)
                        ),
                _ => Ok((CStr::from_ptr(output.r0)
                        .to_string_lossy()
                        .into_owned(), false)
                        ),
            } 
        }
            }
}

pub fn commit(nativeShard: u64, round: u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(n: u64, r: u64) -> i64 > = lib.get(b"commitVc")?;
        Ok(func(nativeShard,round))
    }
}

pub fn verifyProof(input: String, shard:u64,round:u64) -> lib::Result<bool>{
    let c_input = CString::new(input)?;
    let go_str_input = GoString {
      p: c_input.as_ptr(),
      n: c_input.as_bytes().len() as isize,
    };
    let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {        
        let func: lib::Symbol<unsafe extern "C" fn(i: GoString, s:u64, r:u64) -> u8> = lib.get(b"verifyProofVc")?;
        match func(go_str_input, shard, round) {
            1u8 => Ok(true),
            _ => Ok(false)
        } 
    }
}

pub fn demoProofShard() -> lib::Result<String>{
    let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {        
        let func: lib::Symbol<unsafe extern "C" fn() -> *mut ::std::os::raw::c_char> = lib.get(b"demoProof")?;
        Ok(CStr::from_ptr(func()).to_string_lossy().into_owned())
        // match func() {
            // _ => return Ok("as".to_string()),
            // GoString{p:_,n:_} => return Ok("as".to_string()), 
            //return Ok(CStr::from_ptr(p).to_string_lossy().into_owned()),
        // }
    }
}

pub fn demoVerifyShard(input: String) -> lib::Result<u8>{
    let c_input = CString::new(input)?;
    let go_str_input = GoString {
      p: c_input.as_ptr(),
      n: c_input.as_bytes().len() as isize,
    };
    let lib = lib::Library::new("/data/ubuntu/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {        
        let func: lib::Symbol<unsafe extern "C" fn(i: GoString) -> u8> = lib.get(b"demoVerify")?;
        Ok(func(go_str_input))
    }
}
// pub fn update(){
//     unsafe { BenchmarkVCSCommit() };
// }

fn main() {
    // let (tx, rx) = mpsc::channel();
    // let proof = demoProofShard();
    // // let mut proof2;
    // let handle = thread::spawn(move|| {
    //     let proof2 = demoProofShard();
    //     tx.send(proof2).unwrap();
    //     // match proof2 {
    //     //     Ok(p) => println!("{:?}",demoVerifyShard(p)),
    //     //     _ => println!("error fuck"),
    //     // };

    // });
    // thread::sleep(Duration::from_secs(10));
    // match proof {
    //     Ok(p) => println!("proof is {:?}",demoVerifyShard(p)),
    //     _ => println!("error fuck"),
    // };

    // let received = rx.recv().unwrap();
       
    // match received {
    //     Ok(p) => println!("proof2 is {:?}",demoVerifyShard(p)),
    //     _ => println!("error fuck"),
    // };

    // // handle.join().unwrap();
    // // match proof2 {
    // //     Ok(p) => println!("{:?}",demoVerifyShard(p)),
    // //     _ => println!("error fuck"),
    // // };    
}
