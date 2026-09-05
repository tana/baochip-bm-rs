#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_ctrl: SfrCtrl,
    sfr_cfginfo: SfrCfginfo,
    sfr_config: SfrConfig,
    sfr_flevel: SfrFlevel,
    sfr_txf0: SfrTxf0,
    sfr_txf1: SfrTxf1,
    sfr_txf2: SfrTxf2,
    sfr_txf3: SfrTxf3,
    sfr_rxf0: SfrRxf0,
    sfr_rxf1: SfrRxf1,
    sfr_rxf2: SfrRxf2,
    sfr_rxf3: SfrRxf3,
    sfr_elevel: SfrElevel,
    sfr_etype: SfrEtype,
    sfr_event_set: SfrEventSet,
    sfr_event_clr: SfrEventClr,
    sfr_event_status: SfrEventStatus,
    sfr_extclock: SfrExtclock,
    sfr_fifo_clr: SfrFifoClr,
    _reserved19: [u8; 0x04],
    sfr_qdiv0: SfrQdiv0,
    sfr_qdiv1: SfrQdiv1,
    sfr_qdiv2: SfrQdiv2,
    sfr_qdiv3: SfrQdiv3,
    sfr_sync_bypass: SfrSyncBypass,
    sfr_io_oe_inv: SfrIoOeInv,
    sfr_io_o_inv: SfrIoOInv,
    sfr_io_i_inv: SfrIoIInv,
    sfr_irqmask_0: SfrIrqmask0,
    sfr_irqmask_1: SfrIrqmask1,
    sfr_irqmask_2: SfrIrqmask2,
    sfr_irqmask_3: SfrIrqmask3,
    sfr_irq_edge: SfrIrqEdge,
    sfr_dbg_padout: SfrDbgPadout,
    sfr_dbg_padoe: SfrDbgPadoe,
    _reserved34: [u8; 0x04],
    sfr_dbg0: SfrDbg0,
    sfr_dbg1: SfrDbg1,
    sfr_dbg2: SfrDbg2,
    sfr_dbg3: SfrDbg3,
    sfr_mem_gutter: SfrMemGutter,
    sfr_peri_gutter: SfrPeriGutter,
    _reserved40: [u8; 0x08],
    sfr_dmareq_map_cr_evmap0: SfrDmareqMapCrEvmap0,
    sfr_dmareq_map_cr_evmap1: SfrDmareqMapCrEvmap1,
    sfr_dmareq_map_cr_evmap2: SfrDmareqMapCrEvmap2,
    sfr_dmareq_map_cr_evmap3: SfrDmareqMapCrEvmap3,
    sfr_dmareq_map_cr_evmap4: SfrDmareqMapCrEvmap4,
    sfr_dmareq_map_cr_evmap5: SfrDmareqMapCrEvmap5,
    sfr_dmareq_stat_sr_evstat0: SfrDmareqStatSrEvstat0,
    sfr_dmareq_stat_sr_evstat1: SfrDmareqStatSrEvstat1,
    sfr_dmareq_stat_sr_evstat2: SfrDmareqStatSrEvstat2,
    sfr_dmareq_stat_sr_evstat3: SfrDmareqStatSrEvstat3,
    sfr_dmareq_stat_sr_evstat4: SfrDmareqStatSrEvstat4,
    sfr_dmareq_stat_sr_evstat5: SfrDmareqStatSrEvstat5,
    sfr_filter_base_0: SfrFilterBase0,
    sfr_filter_bounds_0: SfrFilterBounds0,
    sfr_filter_base_1: SfrFilterBase1,
    sfr_filter_bounds_1: SfrFilterBounds1,
    sfr_filter_base_2: SfrFilterBase2,
    sfr_filter_bounds_2: SfrFilterBounds2,
    sfr_filter_base_3: SfrFilterBase3,
    sfr_filter_bounds_3: SfrFilterBounds3,
}
impl RegisterBlock {
    #[doc = "0x00 - See `bio_bdma.sv#L488 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L488>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ctrl(&self) -> &SfrCtrl {
        &self.sfr_ctrl
    }
    #[doc = "0x04 - See `bio_bdma.sv#L489 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L489>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfginfo(&self) -> &SfrCfginfo {
        &self.sfr_cfginfo
    }
    #[doc = "0x08 - See `bio_bdma.sv#L490 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L490>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_config(&self) -> &SfrConfig {
        &self.sfr_config
    }
    #[doc = "0x0c - See `bio_bdma.sv#L492 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L492>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_flevel(&self) -> &SfrFlevel {
        &self.sfr_flevel
    }
    #[doc = "0x10 - See `bio_bdma.sv#L493 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L493>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_txf0(&self) -> &SfrTxf0 {
        &self.sfr_txf0
    }
    #[doc = "0x14 - See `bio_bdma.sv#L494 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L494>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_txf1(&self) -> &SfrTxf1 {
        &self.sfr_txf1
    }
    #[doc = "0x18 - See `bio_bdma.sv#L495 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L495>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_txf2(&self) -> &SfrTxf2 {
        &self.sfr_txf2
    }
    #[doc = "0x1c - See `bio_bdma.sv#L496 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L496>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_txf3(&self) -> &SfrTxf3 {
        &self.sfr_txf3
    }
    #[doc = "0x20 - See `bio_bdma.sv#L497 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L497>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rxf0(&self) -> &SfrRxf0 {
        &self.sfr_rxf0
    }
    #[doc = "0x24 - See `bio_bdma.sv#L498 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L498>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rxf1(&self) -> &SfrRxf1 {
        &self.sfr_rxf1
    }
    #[doc = "0x28 - See `bio_bdma.sv#L499 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L499>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rxf2(&self) -> &SfrRxf2 {
        &self.sfr_rxf2
    }
    #[doc = "0x2c - See `bio_bdma.sv#L500 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L500>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rxf3(&self) -> &SfrRxf3 {
        &self.sfr_rxf3
    }
    #[doc = "0x30 - See `bio_bdma.sv#L502 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L502>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_elevel(&self) -> &SfrElevel {
        &self.sfr_elevel
    }
    #[doc = "0x34 - See `bio_bdma.sv#L503 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L503>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_etype(&self) -> &SfrEtype {
        &self.sfr_etype
    }
    #[doc = "0x38 - See `bio_bdma.sv#L504 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L504>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_event_set(&self) -> &SfrEventSet {
        &self.sfr_event_set
    }
    #[doc = "0x3c - See `bio_bdma.sv#L505 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L505>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_event_clr(&self) -> &SfrEventClr {
        &self.sfr_event_clr
    }
    #[doc = "0x40 - See `bio_bdma.sv#L506 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L506>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_event_status(&self) -> &SfrEventStatus {
        &self.sfr_event_status
    }
    #[doc = "0x44 - See `bio_bdma.sv#L508 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L508>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_extclock(&self) -> &SfrExtclock {
        &self.sfr_extclock
    }
    #[doc = "0x48 - See `bio_bdma.sv#L509 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L509>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_fifo_clr(&self) -> &SfrFifoClr {
        &self.sfr_fifo_clr
    }
    #[doc = "0x50 - See `bio_bdma.sv#L511 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L511>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_qdiv0(&self) -> &SfrQdiv0 {
        &self.sfr_qdiv0
    }
    #[doc = "0x54 - See `bio_bdma.sv#L512 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L512>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_qdiv1(&self) -> &SfrQdiv1 {
        &self.sfr_qdiv1
    }
    #[doc = "0x58 - See `bio_bdma.sv#L513 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L513>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_qdiv2(&self) -> &SfrQdiv2 {
        &self.sfr_qdiv2
    }
    #[doc = "0x5c - See `bio_bdma.sv#L514 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L514>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_qdiv3(&self) -> &SfrQdiv3 {
        &self.sfr_qdiv3
    }
    #[doc = "0x60 - See `bio_bdma.sv#L516 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L516>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sync_bypass(&self) -> &SfrSyncBypass {
        &self.sfr_sync_bypass
    }
    #[doc = "0x64 - See `bio_bdma.sv#L517 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L517>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_io_oe_inv(&self) -> &SfrIoOeInv {
        &self.sfr_io_oe_inv
    }
    #[doc = "0x68 - See `bio_bdma.sv#L518 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L518>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_io_o_inv(&self) -> &SfrIoOInv {
        &self.sfr_io_o_inv
    }
    #[doc = "0x6c - See `bio_bdma.sv#L519 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L519>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_io_i_inv(&self) -> &SfrIoIInv {
        &self.sfr_io_i_inv
    }
    #[doc = "0x70 - See `bio_bdma.sv#L521 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L521>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_irqmask_0(&self) -> &SfrIrqmask0 {
        &self.sfr_irqmask_0
    }
    #[doc = "0x74 - See `bio_bdma.sv#L522 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L522>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_irqmask_1(&self) -> &SfrIrqmask1 {
        &self.sfr_irqmask_1
    }
    #[doc = "0x78 - See `bio_bdma.sv#L523 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L523>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_irqmask_2(&self) -> &SfrIrqmask2 {
        &self.sfr_irqmask_2
    }
    #[doc = "0x7c - See `bio_bdma.sv#L524 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L524>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_irqmask_3(&self) -> &SfrIrqmask3 {
        &self.sfr_irqmask_3
    }
    #[doc = "0x80 - See `bio_bdma.sv#L525 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L525>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_irq_edge(&self) -> &SfrIrqEdge {
        &self.sfr_irq_edge
    }
    #[doc = "0x84 - See `bio_bdma.sv#L526 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L526>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dbg_padout(&self) -> &SfrDbgPadout {
        &self.sfr_dbg_padout
    }
    #[doc = "0x88 - See `bio_bdma.sv#L527 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L527>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dbg_padoe(&self) -> &SfrDbgPadoe {
        &self.sfr_dbg_padoe
    }
    #[doc = "0x90 - See `bio_bdma.sv#L529 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L529>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dbg0(&self) -> &SfrDbg0 {
        &self.sfr_dbg0
    }
    #[doc = "0x94 - See `bio_bdma.sv#L530 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L530>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dbg1(&self) -> &SfrDbg1 {
        &self.sfr_dbg1
    }
    #[doc = "0x98 - See `bio_bdma.sv#L531 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L531>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dbg2(&self) -> &SfrDbg2 {
        &self.sfr_dbg2
    }
    #[doc = "0x9c - See `bio_bdma.sv#L532 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L532>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dbg3(&self) -> &SfrDbg3 {
        &self.sfr_dbg3
    }
    #[doc = "0xa0 - See `bio_bdma.sv#L535 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L535>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mem_gutter(&self) -> &SfrMemGutter {
        &self.sfr_mem_gutter
    }
    #[doc = "0xa4 - See `bio_bdma.sv#L536 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L536>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_peri_gutter(&self) -> &SfrPeriGutter {
        &self.sfr_peri_gutter
    }
    #[doc = "0xb0 - See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_map_cr_evmap0(&self) -> &SfrDmareqMapCrEvmap0 {
        &self.sfr_dmareq_map_cr_evmap0
    }
    #[doc = "0xb4 - See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_map_cr_evmap1(&self) -> &SfrDmareqMapCrEvmap1 {
        &self.sfr_dmareq_map_cr_evmap1
    }
    #[doc = "0xb8 - See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_map_cr_evmap2(&self) -> &SfrDmareqMapCrEvmap2 {
        &self.sfr_dmareq_map_cr_evmap2
    }
    #[doc = "0xbc - See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_map_cr_evmap3(&self) -> &SfrDmareqMapCrEvmap3 {
        &self.sfr_dmareq_map_cr_evmap3
    }
    #[doc = "0xc0 - See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_map_cr_evmap4(&self) -> &SfrDmareqMapCrEvmap4 {
        &self.sfr_dmareq_map_cr_evmap4
    }
    #[doc = "0xc4 - See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_map_cr_evmap5(&self) -> &SfrDmareqMapCrEvmap5 {
        &self.sfr_dmareq_map_cr_evmap5
    }
    #[doc = "0xc8 - See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_stat_sr_evstat0(&self) -> &SfrDmareqStatSrEvstat0 {
        &self.sfr_dmareq_stat_sr_evstat0
    }
    #[doc = "0xcc - See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_stat_sr_evstat1(&self) -> &SfrDmareqStatSrEvstat1 {
        &self.sfr_dmareq_stat_sr_evstat1
    }
    #[doc = "0xd0 - See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_stat_sr_evstat2(&self) -> &SfrDmareqStatSrEvstat2 {
        &self.sfr_dmareq_stat_sr_evstat2
    }
    #[doc = "0xd4 - See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_stat_sr_evstat3(&self) -> &SfrDmareqStatSrEvstat3 {
        &self.sfr_dmareq_stat_sr_evstat3
    }
    #[doc = "0xd8 - See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_stat_sr_evstat4(&self) -> &SfrDmareqStatSrEvstat4 {
        &self.sfr_dmareq_stat_sr_evstat4
    }
    #[doc = "0xdc - See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dmareq_stat_sr_evstat5(&self) -> &SfrDmareqStatSrEvstat5 {
        &self.sfr_dmareq_stat_sr_evstat5
    }
    #[doc = "0xe0 - See `bio_bdma.sv#L593 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L593>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_filter_base_0(&self) -> &SfrFilterBase0 {
        &self.sfr_filter_base_0
    }
    #[doc = "0xe4 - See `bio_bdma.sv#L594 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L594>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_filter_bounds_0(&self) -> &SfrFilterBounds0 {
        &self.sfr_filter_bounds_0
    }
    #[doc = "0xe8 - See `bio_bdma.sv#L595 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L595>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_filter_base_1(&self) -> &SfrFilterBase1 {
        &self.sfr_filter_base_1
    }
    #[doc = "0xec - See `bio_bdma.sv#L596 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L596>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_filter_bounds_1(&self) -> &SfrFilterBounds1 {
        &self.sfr_filter_bounds_1
    }
    #[doc = "0xf0 - See `bio_bdma.sv#L597 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L597>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_filter_base_2(&self) -> &SfrFilterBase2 {
        &self.sfr_filter_base_2
    }
    #[doc = "0xf4 - See `bio_bdma.sv#L598 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L598>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_filter_bounds_2(&self) -> &SfrFilterBounds2 {
        &self.sfr_filter_bounds_2
    }
    #[doc = "0xf8 - See `bio_bdma.sv#L599 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L599>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_filter_base_3(&self) -> &SfrFilterBase3 {
        &self.sfr_filter_base_3
    }
    #[doc = "0xfc - See `bio_bdma.sv#L600 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L600>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_filter_bounds_3(&self) -> &SfrFilterBounds3 {
        &self.sfr_filter_bounds_3
    }
}
#[doc = "SFR_CTRL (rw) register accessor: See `bio_bdma.sv#L488 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L488>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ctrl`] module"]
#[doc(alias = "SFR_CTRL")]
pub type SfrCtrl = crate::Reg<sfr_ctrl::SfrCtrlSpec>;
#[doc = "See `bio_bdma.sv#L488 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L488>`__ (line numbers are approximate)"]
pub mod sfr_ctrl;
#[doc = "SFR_CFGINFO (rw) register accessor: See `bio_bdma.sv#L489 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L489>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfginfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfginfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfginfo`] module"]
#[doc(alias = "SFR_CFGINFO")]
pub type SfrCfginfo = crate::Reg<sfr_cfginfo::SfrCfginfoSpec>;
#[doc = "See `bio_bdma.sv#L489 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L489>`__ (line numbers are approximate)"]
pub mod sfr_cfginfo;
#[doc = "SFR_CONFIG (rw) register accessor: See `bio_bdma.sv#L490 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L490>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_config`] module"]
#[doc(alias = "SFR_CONFIG")]
pub type SfrConfig = crate::Reg<sfr_config::SfrConfigSpec>;
#[doc = "See `bio_bdma.sv#L490 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L490>`__ (line numbers are approximate)"]
pub mod sfr_config;
#[doc = "SFR_FLEVEL (rw) register accessor: See `bio_bdma.sv#L492 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L492>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_flevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_flevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_flevel`] module"]
#[doc(alias = "SFR_FLEVEL")]
pub type SfrFlevel = crate::Reg<sfr_flevel::SfrFlevelSpec>;
#[doc = "See `bio_bdma.sv#L492 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L492>`__ (line numbers are approximate)"]
pub mod sfr_flevel;
#[doc = "SFR_TXF0 (rw) register accessor: See `bio_bdma.sv#L493 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L493>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_txf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_txf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_txf0`] module"]
#[doc(alias = "SFR_TXF0")]
pub type SfrTxf0 = crate::Reg<sfr_txf0::SfrTxf0Spec>;
#[doc = "See `bio_bdma.sv#L493 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L493>`__ (line numbers are approximate)"]
pub mod sfr_txf0;
#[doc = "SFR_TXF1 (rw) register accessor: See `bio_bdma.sv#L494 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L494>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_txf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_txf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_txf1`] module"]
#[doc(alias = "SFR_TXF1")]
pub type SfrTxf1 = crate::Reg<sfr_txf1::SfrTxf1Spec>;
#[doc = "See `bio_bdma.sv#L494 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L494>`__ (line numbers are approximate)"]
pub mod sfr_txf1;
#[doc = "SFR_TXF2 (rw) register accessor: See `bio_bdma.sv#L495 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L495>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_txf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_txf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_txf2`] module"]
#[doc(alias = "SFR_TXF2")]
pub type SfrTxf2 = crate::Reg<sfr_txf2::SfrTxf2Spec>;
#[doc = "See `bio_bdma.sv#L495 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L495>`__ (line numbers are approximate)"]
pub mod sfr_txf2;
#[doc = "SFR_TXF3 (rw) register accessor: See `bio_bdma.sv#L496 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L496>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_txf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_txf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_txf3`] module"]
#[doc(alias = "SFR_TXF3")]
pub type SfrTxf3 = crate::Reg<sfr_txf3::SfrTxf3Spec>;
#[doc = "See `bio_bdma.sv#L496 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L496>`__ (line numbers are approximate)"]
pub mod sfr_txf3;
#[doc = "SFR_RXF0 (rw) register accessor: See `bio_bdma.sv#L497 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L497>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rxf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rxf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rxf0`] module"]
#[doc(alias = "SFR_RXF0")]
pub type SfrRxf0 = crate::Reg<sfr_rxf0::SfrRxf0Spec>;
#[doc = "See `bio_bdma.sv#L497 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L497>`__ (line numbers are approximate)"]
pub mod sfr_rxf0;
#[doc = "SFR_RXF1 (rw) register accessor: See `bio_bdma.sv#L498 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L498>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rxf1`] module"]
#[doc(alias = "SFR_RXF1")]
pub type SfrRxf1 = crate::Reg<sfr_rxf1::SfrRxf1Spec>;
#[doc = "See `bio_bdma.sv#L498 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L498>`__ (line numbers are approximate)"]
pub mod sfr_rxf1;
#[doc = "SFR_RXF2 (rw) register accessor: See `bio_bdma.sv#L499 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L499>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rxf2`] module"]
#[doc(alias = "SFR_RXF2")]
pub type SfrRxf2 = crate::Reg<sfr_rxf2::SfrRxf2Spec>;
#[doc = "See `bio_bdma.sv#L499 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L499>`__ (line numbers are approximate)"]
pub mod sfr_rxf2;
#[doc = "SFR_RXF3 (rw) register accessor: See `bio_bdma.sv#L500 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L500>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rxf3`] module"]
#[doc(alias = "SFR_RXF3")]
pub type SfrRxf3 = crate::Reg<sfr_rxf3::SfrRxf3Spec>;
#[doc = "See `bio_bdma.sv#L500 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L500>`__ (line numbers are approximate)"]
pub mod sfr_rxf3;
#[doc = "SFR_ELEVEL (rw) register accessor: See `bio_bdma.sv#L502 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L502>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_elevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_elevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_elevel`] module"]
#[doc(alias = "SFR_ELEVEL")]
pub type SfrElevel = crate::Reg<sfr_elevel::SfrElevelSpec>;
#[doc = "See `bio_bdma.sv#L502 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L502>`__ (line numbers are approximate)"]
pub mod sfr_elevel;
#[doc = "SFR_ETYPE (rw) register accessor: See `bio_bdma.sv#L503 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L503>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_etype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_etype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_etype`] module"]
#[doc(alias = "SFR_ETYPE")]
pub type SfrEtype = crate::Reg<sfr_etype::SfrEtypeSpec>;
#[doc = "See `bio_bdma.sv#L503 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L503>`__ (line numbers are approximate)"]
pub mod sfr_etype;
#[doc = "SFR_EVENT_SET (rw) register accessor: See `bio_bdma.sv#L504 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L504>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_event_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_event_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_event_set`] module"]
#[doc(alias = "SFR_EVENT_SET")]
pub type SfrEventSet = crate::Reg<sfr_event_set::SfrEventSetSpec>;
#[doc = "See `bio_bdma.sv#L504 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L504>`__ (line numbers are approximate)"]
pub mod sfr_event_set;
#[doc = "SFR_EVENT_CLR (rw) register accessor: See `bio_bdma.sv#L505 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L505>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_event_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_event_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_event_clr`] module"]
#[doc(alias = "SFR_EVENT_CLR")]
pub type SfrEventClr = crate::Reg<sfr_event_clr::SfrEventClrSpec>;
#[doc = "See `bio_bdma.sv#L505 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L505>`__ (line numbers are approximate)"]
pub mod sfr_event_clr;
#[doc = "SFR_EVENT_STATUS (rw) register accessor: See `bio_bdma.sv#L506 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L506>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_event_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_event_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_event_status`] module"]
#[doc(alias = "SFR_EVENT_STATUS")]
pub type SfrEventStatus = crate::Reg<sfr_event_status::SfrEventStatusSpec>;
#[doc = "See `bio_bdma.sv#L506 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L506>`__ (line numbers are approximate)"]
pub mod sfr_event_status;
#[doc = "SFR_EXTCLOCK (rw) register accessor: See `bio_bdma.sv#L508 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L508>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_extclock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_extclock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_extclock`] module"]
#[doc(alias = "SFR_EXTCLOCK")]
pub type SfrExtclock = crate::Reg<sfr_extclock::SfrExtclockSpec>;
#[doc = "See `bio_bdma.sv#L508 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L508>`__ (line numbers are approximate)"]
pub mod sfr_extclock;
#[doc = "SFR_FIFO_CLR (rw) register accessor: See `bio_bdma.sv#L509 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L509>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fifo_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fifo_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_fifo_clr`] module"]
#[doc(alias = "SFR_FIFO_CLR")]
pub type SfrFifoClr = crate::Reg<sfr_fifo_clr::SfrFifoClrSpec>;
#[doc = "See `bio_bdma.sv#L509 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L509>`__ (line numbers are approximate)"]
pub mod sfr_fifo_clr;
#[doc = "SFR_QDIV0 (rw) register accessor: See `bio_bdma.sv#L511 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L511>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_qdiv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_qdiv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_qdiv0`] module"]
#[doc(alias = "SFR_QDIV0")]
pub type SfrQdiv0 = crate::Reg<sfr_qdiv0::SfrQdiv0Spec>;
#[doc = "See `bio_bdma.sv#L511 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L511>`__ (line numbers are approximate)"]
pub mod sfr_qdiv0;
#[doc = "SFR_QDIV1 (rw) register accessor: See `bio_bdma.sv#L512 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L512>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_qdiv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_qdiv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_qdiv1`] module"]
#[doc(alias = "SFR_QDIV1")]
pub type SfrQdiv1 = crate::Reg<sfr_qdiv1::SfrQdiv1Spec>;
#[doc = "See `bio_bdma.sv#L512 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L512>`__ (line numbers are approximate)"]
pub mod sfr_qdiv1;
#[doc = "SFR_QDIV2 (rw) register accessor: See `bio_bdma.sv#L513 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L513>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_qdiv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_qdiv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_qdiv2`] module"]
#[doc(alias = "SFR_QDIV2")]
pub type SfrQdiv2 = crate::Reg<sfr_qdiv2::SfrQdiv2Spec>;
#[doc = "See `bio_bdma.sv#L513 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L513>`__ (line numbers are approximate)"]
pub mod sfr_qdiv2;
#[doc = "SFR_QDIV3 (rw) register accessor: See `bio_bdma.sv#L514 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L514>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_qdiv3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_qdiv3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_qdiv3`] module"]
#[doc(alias = "SFR_QDIV3")]
pub type SfrQdiv3 = crate::Reg<sfr_qdiv3::SfrQdiv3Spec>;
#[doc = "See `bio_bdma.sv#L514 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L514>`__ (line numbers are approximate)"]
pub mod sfr_qdiv3;
#[doc = "SFR_SYNC_BYPASS (rw) register accessor: See `bio_bdma.sv#L516 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L516>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sync_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sync_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sync_bypass`] module"]
#[doc(alias = "SFR_SYNC_BYPASS")]
pub type SfrSyncBypass = crate::Reg<sfr_sync_bypass::SfrSyncBypassSpec>;
#[doc = "See `bio_bdma.sv#L516 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L516>`__ (line numbers are approximate)"]
pub mod sfr_sync_bypass;
#[doc = "SFR_IO_OE_INV (rw) register accessor: See `bio_bdma.sv#L517 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L517>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_io_oe_inv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_io_oe_inv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_io_oe_inv`] module"]
#[doc(alias = "SFR_IO_OE_INV")]
pub type SfrIoOeInv = crate::Reg<sfr_io_oe_inv::SfrIoOeInvSpec>;
#[doc = "See `bio_bdma.sv#L517 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L517>`__ (line numbers are approximate)"]
pub mod sfr_io_oe_inv;
#[doc = "SFR_IO_O_INV (rw) register accessor: See `bio_bdma.sv#L518 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L518>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_io_o_inv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_io_o_inv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_io_o_inv`] module"]
#[doc(alias = "SFR_IO_O_INV")]
pub type SfrIoOInv = crate::Reg<sfr_io_o_inv::SfrIoOInvSpec>;
#[doc = "See `bio_bdma.sv#L518 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L518>`__ (line numbers are approximate)"]
pub mod sfr_io_o_inv;
#[doc = "SFR_IO_I_INV (rw) register accessor: See `bio_bdma.sv#L519 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L519>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_io_i_inv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_io_i_inv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_io_i_inv`] module"]
#[doc(alias = "SFR_IO_I_INV")]
pub type SfrIoIInv = crate::Reg<sfr_io_i_inv::SfrIoIInvSpec>;
#[doc = "See `bio_bdma.sv#L519 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L519>`__ (line numbers are approximate)"]
pub mod sfr_io_i_inv;
#[doc = "SFR_IRQMASK_0 (rw) register accessor: See `bio_bdma.sv#L521 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L521>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_irqmask_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_irqmask_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_irqmask_0`] module"]
#[doc(alias = "SFR_IRQMASK_0")]
pub type SfrIrqmask0 = crate::Reg<sfr_irqmask_0::SfrIrqmask0Spec>;
#[doc = "See `bio_bdma.sv#L521 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L521>`__ (line numbers are approximate)"]
pub mod sfr_irqmask_0;
#[doc = "SFR_IRQMASK_1 (rw) register accessor: See `bio_bdma.sv#L522 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L522>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_irqmask_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_irqmask_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_irqmask_1`] module"]
#[doc(alias = "SFR_IRQMASK_1")]
pub type SfrIrqmask1 = crate::Reg<sfr_irqmask_1::SfrIrqmask1Spec>;
#[doc = "See `bio_bdma.sv#L522 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L522>`__ (line numbers are approximate)"]
pub mod sfr_irqmask_1;
#[doc = "SFR_IRQMASK_2 (rw) register accessor: See `bio_bdma.sv#L523 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L523>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_irqmask_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_irqmask_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_irqmask_2`] module"]
#[doc(alias = "SFR_IRQMASK_2")]
pub type SfrIrqmask2 = crate::Reg<sfr_irqmask_2::SfrIrqmask2Spec>;
#[doc = "See `bio_bdma.sv#L523 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L523>`__ (line numbers are approximate)"]
pub mod sfr_irqmask_2;
#[doc = "SFR_IRQMASK_3 (rw) register accessor: See `bio_bdma.sv#L524 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L524>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_irqmask_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_irqmask_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_irqmask_3`] module"]
#[doc(alias = "SFR_IRQMASK_3")]
pub type SfrIrqmask3 = crate::Reg<sfr_irqmask_3::SfrIrqmask3Spec>;
#[doc = "See `bio_bdma.sv#L524 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L524>`__ (line numbers are approximate)"]
pub mod sfr_irqmask_3;
#[doc = "SFR_IRQ_EDGE (rw) register accessor: See `bio_bdma.sv#L525 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L525>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_irq_edge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_irq_edge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_irq_edge`] module"]
#[doc(alias = "SFR_IRQ_EDGE")]
pub type SfrIrqEdge = crate::Reg<sfr_irq_edge::SfrIrqEdgeSpec>;
#[doc = "See `bio_bdma.sv#L525 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L525>`__ (line numbers are approximate)"]
pub mod sfr_irq_edge;
#[doc = "SFR_DBG_PADOUT (rw) register accessor: See `bio_bdma.sv#L526 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L526>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dbg_padout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dbg_padout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dbg_padout`] module"]
#[doc(alias = "SFR_DBG_PADOUT")]
pub type SfrDbgPadout = crate::Reg<sfr_dbg_padout::SfrDbgPadoutSpec>;
#[doc = "See `bio_bdma.sv#L526 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L526>`__ (line numbers are approximate)"]
pub mod sfr_dbg_padout;
#[doc = "SFR_DBG_PADOE (rw) register accessor: See `bio_bdma.sv#L527 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L527>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dbg_padoe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dbg_padoe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dbg_padoe`] module"]
#[doc(alias = "SFR_DBG_PADOE")]
pub type SfrDbgPadoe = crate::Reg<sfr_dbg_padoe::SfrDbgPadoeSpec>;
#[doc = "See `bio_bdma.sv#L527 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L527>`__ (line numbers are approximate)"]
pub mod sfr_dbg_padoe;
#[doc = "SFR_DBG0 (rw) register accessor: See `bio_bdma.sv#L529 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L529>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dbg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dbg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dbg0`] module"]
#[doc(alias = "SFR_DBG0")]
pub type SfrDbg0 = crate::Reg<sfr_dbg0::SfrDbg0Spec>;
#[doc = "See `bio_bdma.sv#L529 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L529>`__ (line numbers are approximate)"]
pub mod sfr_dbg0;
#[doc = "SFR_DBG1 (rw) register accessor: See `bio_bdma.sv#L530 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L530>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dbg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dbg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dbg1`] module"]
#[doc(alias = "SFR_DBG1")]
pub type SfrDbg1 = crate::Reg<sfr_dbg1::SfrDbg1Spec>;
#[doc = "See `bio_bdma.sv#L530 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L530>`__ (line numbers are approximate)"]
pub mod sfr_dbg1;
#[doc = "SFR_DBG2 (rw) register accessor: See `bio_bdma.sv#L531 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L531>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dbg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dbg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dbg2`] module"]
#[doc(alias = "SFR_DBG2")]
pub type SfrDbg2 = crate::Reg<sfr_dbg2::SfrDbg2Spec>;
#[doc = "See `bio_bdma.sv#L531 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L531>`__ (line numbers are approximate)"]
pub mod sfr_dbg2;
#[doc = "SFR_DBG3 (rw) register accessor: See `bio_bdma.sv#L532 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L532>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dbg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dbg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dbg3`] module"]
#[doc(alias = "SFR_DBG3")]
pub type SfrDbg3 = crate::Reg<sfr_dbg3::SfrDbg3Spec>;
#[doc = "See `bio_bdma.sv#L532 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L532>`__ (line numbers are approximate)"]
pub mod sfr_dbg3;
#[doc = "SFR_MEM_GUTTER (rw) register accessor: See `bio_bdma.sv#L535 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L535>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mem_gutter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mem_gutter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mem_gutter`] module"]
#[doc(alias = "SFR_MEM_GUTTER")]
pub type SfrMemGutter = crate::Reg<sfr_mem_gutter::SfrMemGutterSpec>;
#[doc = "See `bio_bdma.sv#L535 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L535>`__ (line numbers are approximate)"]
pub mod sfr_mem_gutter;
#[doc = "SFR_PERI_GUTTER (rw) register accessor: See `bio_bdma.sv#L536 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L536>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_peri_gutter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_peri_gutter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_peri_gutter`] module"]
#[doc(alias = "SFR_PERI_GUTTER")]
pub type SfrPeriGutter = crate::Reg<sfr_peri_gutter::SfrPeriGutterSpec>;
#[doc = "See `bio_bdma.sv#L536 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L536>`__ (line numbers are approximate)"]
pub mod sfr_peri_gutter;
#[doc = "SFR_DMAREQ_MAP_CR_EVMAP0 (rw) register accessor: See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_map_cr_evmap0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_map_cr_evmap0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_map_cr_evmap0`] module"]
#[doc(alias = "SFR_DMAREQ_MAP_CR_EVMAP0")]
pub type SfrDmareqMapCrEvmap0 = crate::Reg<sfr_dmareq_map_cr_evmap0::SfrDmareqMapCrEvmap0Spec>;
#[doc = "See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_map_cr_evmap0;
#[doc = "SFR_DMAREQ_MAP_CR_EVMAP1 (rw) register accessor: See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_map_cr_evmap1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_map_cr_evmap1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_map_cr_evmap1`] module"]
#[doc(alias = "SFR_DMAREQ_MAP_CR_EVMAP1")]
pub type SfrDmareqMapCrEvmap1 = crate::Reg<sfr_dmareq_map_cr_evmap1::SfrDmareqMapCrEvmap1Spec>;
#[doc = "See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_map_cr_evmap1;
#[doc = "SFR_DMAREQ_MAP_CR_EVMAP2 (rw) register accessor: See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_map_cr_evmap2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_map_cr_evmap2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_map_cr_evmap2`] module"]
#[doc(alias = "SFR_DMAREQ_MAP_CR_EVMAP2")]
pub type SfrDmareqMapCrEvmap2 = crate::Reg<sfr_dmareq_map_cr_evmap2::SfrDmareqMapCrEvmap2Spec>;
#[doc = "See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_map_cr_evmap2;
#[doc = "SFR_DMAREQ_MAP_CR_EVMAP3 (rw) register accessor: See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_map_cr_evmap3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_map_cr_evmap3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_map_cr_evmap3`] module"]
#[doc(alias = "SFR_DMAREQ_MAP_CR_EVMAP3")]
pub type SfrDmareqMapCrEvmap3 = crate::Reg<sfr_dmareq_map_cr_evmap3::SfrDmareqMapCrEvmap3Spec>;
#[doc = "See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_map_cr_evmap3;
#[doc = "SFR_DMAREQ_MAP_CR_EVMAP4 (rw) register accessor: See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_map_cr_evmap4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_map_cr_evmap4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_map_cr_evmap4`] module"]
#[doc(alias = "SFR_DMAREQ_MAP_CR_EVMAP4")]
pub type SfrDmareqMapCrEvmap4 = crate::Reg<sfr_dmareq_map_cr_evmap4::SfrDmareqMapCrEvmap4Spec>;
#[doc = "See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_map_cr_evmap4;
#[doc = "SFR_DMAREQ_MAP_CR_EVMAP5 (rw) register accessor: See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_map_cr_evmap5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_map_cr_evmap5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_map_cr_evmap5`] module"]
#[doc(alias = "SFR_DMAREQ_MAP_CR_EVMAP5")]
pub type SfrDmareqMapCrEvmap5 = crate::Reg<sfr_dmareq_map_cr_evmap5::SfrDmareqMapCrEvmap5Spec>;
#[doc = "See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_map_cr_evmap5;
#[doc = "SFR_DMAREQ_STAT_SR_EVSTAT0 (rw) register accessor: See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_stat_sr_evstat0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_stat_sr_evstat0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_stat_sr_evstat0`] module"]
#[doc(alias = "SFR_DMAREQ_STAT_SR_EVSTAT0")]
pub type SfrDmareqStatSrEvstat0 =
    crate::Reg<sfr_dmareq_stat_sr_evstat0::SfrDmareqStatSrEvstat0Spec>;
#[doc = "See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_stat_sr_evstat0;
#[doc = "SFR_DMAREQ_STAT_SR_EVSTAT1 (rw) register accessor: See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_stat_sr_evstat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_stat_sr_evstat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_stat_sr_evstat1`] module"]
#[doc(alias = "SFR_DMAREQ_STAT_SR_EVSTAT1")]
pub type SfrDmareqStatSrEvstat1 =
    crate::Reg<sfr_dmareq_stat_sr_evstat1::SfrDmareqStatSrEvstat1Spec>;
#[doc = "See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_stat_sr_evstat1;
#[doc = "SFR_DMAREQ_STAT_SR_EVSTAT2 (rw) register accessor: See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_stat_sr_evstat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_stat_sr_evstat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_stat_sr_evstat2`] module"]
#[doc(alias = "SFR_DMAREQ_STAT_SR_EVSTAT2")]
pub type SfrDmareqStatSrEvstat2 =
    crate::Reg<sfr_dmareq_stat_sr_evstat2::SfrDmareqStatSrEvstat2Spec>;
#[doc = "See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_stat_sr_evstat2;
#[doc = "SFR_DMAREQ_STAT_SR_EVSTAT3 (rw) register accessor: See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_stat_sr_evstat3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_stat_sr_evstat3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_stat_sr_evstat3`] module"]
#[doc(alias = "SFR_DMAREQ_STAT_SR_EVSTAT3")]
pub type SfrDmareqStatSrEvstat3 =
    crate::Reg<sfr_dmareq_stat_sr_evstat3::SfrDmareqStatSrEvstat3Spec>;
#[doc = "See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_stat_sr_evstat3;
#[doc = "SFR_DMAREQ_STAT_SR_EVSTAT4 (rw) register accessor: See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_stat_sr_evstat4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_stat_sr_evstat4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_stat_sr_evstat4`] module"]
#[doc(alias = "SFR_DMAREQ_STAT_SR_EVSTAT4")]
pub type SfrDmareqStatSrEvstat4 =
    crate::Reg<sfr_dmareq_stat_sr_evstat4::SfrDmareqStatSrEvstat4Spec>;
#[doc = "See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_stat_sr_evstat4;
#[doc = "SFR_DMAREQ_STAT_SR_EVSTAT5 (rw) register accessor: See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_stat_sr_evstat5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_stat_sr_evstat5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dmareq_stat_sr_evstat5`] module"]
#[doc(alias = "SFR_DMAREQ_STAT_SR_EVSTAT5")]
pub type SfrDmareqStatSrEvstat5 =
    crate::Reg<sfr_dmareq_stat_sr_evstat5::SfrDmareqStatSrEvstat5Spec>;
#[doc = "See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)"]
pub mod sfr_dmareq_stat_sr_evstat5;
#[doc = "SFR_FILTER_BASE_0 (rw) register accessor: See `bio_bdma.sv#L593 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L593>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_base_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_base_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_filter_base_0`] module"]
#[doc(alias = "SFR_FILTER_BASE_0")]
pub type SfrFilterBase0 = crate::Reg<sfr_filter_base_0::SfrFilterBase0Spec>;
#[doc = "See `bio_bdma.sv#L593 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L593>`__ (line numbers are approximate)"]
pub mod sfr_filter_base_0;
#[doc = "SFR_FILTER_BOUNDS_0 (rw) register accessor: See `bio_bdma.sv#L594 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L594>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_bounds_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_bounds_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_filter_bounds_0`] module"]
#[doc(alias = "SFR_FILTER_BOUNDS_0")]
pub type SfrFilterBounds0 = crate::Reg<sfr_filter_bounds_0::SfrFilterBounds0Spec>;
#[doc = "See `bio_bdma.sv#L594 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L594>`__ (line numbers are approximate)"]
pub mod sfr_filter_bounds_0;
#[doc = "SFR_FILTER_BASE_1 (rw) register accessor: See `bio_bdma.sv#L595 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L595>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_base_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_base_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_filter_base_1`] module"]
#[doc(alias = "SFR_FILTER_BASE_1")]
pub type SfrFilterBase1 = crate::Reg<sfr_filter_base_1::SfrFilterBase1Spec>;
#[doc = "See `bio_bdma.sv#L595 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L595>`__ (line numbers are approximate)"]
pub mod sfr_filter_base_1;
#[doc = "SFR_FILTER_BOUNDS_1 (rw) register accessor: See `bio_bdma.sv#L596 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L596>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_bounds_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_bounds_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_filter_bounds_1`] module"]
#[doc(alias = "SFR_FILTER_BOUNDS_1")]
pub type SfrFilterBounds1 = crate::Reg<sfr_filter_bounds_1::SfrFilterBounds1Spec>;
#[doc = "See `bio_bdma.sv#L596 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L596>`__ (line numbers are approximate)"]
pub mod sfr_filter_bounds_1;
#[doc = "SFR_FILTER_BASE_2 (rw) register accessor: See `bio_bdma.sv#L597 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L597>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_base_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_base_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_filter_base_2`] module"]
#[doc(alias = "SFR_FILTER_BASE_2")]
pub type SfrFilterBase2 = crate::Reg<sfr_filter_base_2::SfrFilterBase2Spec>;
#[doc = "See `bio_bdma.sv#L597 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L597>`__ (line numbers are approximate)"]
pub mod sfr_filter_base_2;
#[doc = "SFR_FILTER_BOUNDS_2 (rw) register accessor: See `bio_bdma.sv#L598 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L598>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_bounds_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_bounds_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_filter_bounds_2`] module"]
#[doc(alias = "SFR_FILTER_BOUNDS_2")]
pub type SfrFilterBounds2 = crate::Reg<sfr_filter_bounds_2::SfrFilterBounds2Spec>;
#[doc = "See `bio_bdma.sv#L598 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L598>`__ (line numbers are approximate)"]
pub mod sfr_filter_bounds_2;
#[doc = "SFR_FILTER_BASE_3 (rw) register accessor: See `bio_bdma.sv#L599 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L599>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_base_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_base_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_filter_base_3`] module"]
#[doc(alias = "SFR_FILTER_BASE_3")]
pub type SfrFilterBase3 = crate::Reg<sfr_filter_base_3::SfrFilterBase3Spec>;
#[doc = "See `bio_bdma.sv#L599 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L599>`__ (line numbers are approximate)"]
pub mod sfr_filter_base_3;
#[doc = "SFR_FILTER_BOUNDS_3 (rw) register accessor: See `bio_bdma.sv#L600 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L600>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_bounds_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_bounds_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_filter_bounds_3`] module"]
#[doc(alias = "SFR_FILTER_BOUNDS_3")]
pub type SfrFilterBounds3 = crate::Reg<sfr_filter_bounds_3::SfrFilterBounds3Spec>;
#[doc = "See `bio_bdma.sv#L600 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L600>`__ (line numbers are approximate)"]
pub mod sfr_filter_bounds_3;
