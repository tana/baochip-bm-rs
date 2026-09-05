#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_afsel_crafsel0: SfrAfselCrafsel0,
    sfr_afsel_crafsel1: SfrAfselCrafsel1,
    sfr_afsel_crafsel2: SfrAfselCrafsel2,
    sfr_afsel_crafsel3: SfrAfselCrafsel3,
    sfr_afsel_crafsel4: SfrAfselCrafsel4,
    sfr_afsel_crafsel5: SfrAfselCrafsel5,
    sfr_afsel_crafsel6: SfrAfselCrafsel6,
    sfr_afsel_crafsel7: SfrAfselCrafsel7,
    sfr_afsel_crafsel8: SfrAfselCrafsel8,
    sfr_afsel_crafsel9: SfrAfselCrafsel9,
    sfr_afsel_crafsel10: SfrAfselCrafsel10,
    sfr_afsel_crafsel11: SfrAfselCrafsel11,
    _reserved12: [u8; 0xd0],
    sfr_intcr_crint0: SfrIntcrCrint0,
    sfr_intcr_crint1: SfrIntcrCrint1,
    sfr_intcr_crint2: SfrIntcrCrint2,
    sfr_intcr_crint3: SfrIntcrCrint3,
    sfr_intcr_crint4: SfrIntcrCrint4,
    sfr_intcr_crint5: SfrIntcrCrint5,
    sfr_intcr_crint6: SfrIntcrCrint6,
    sfr_intcr_crint7: SfrIntcrCrint7,
    sfr_intfr: SfrIntfr,
    _reserved21: [u8; 0x0c],
    sfr_gpioout_crgo0: SfrGpiooutCrgo0,
    sfr_gpioout_crgo1: SfrGpiooutCrgo1,
    sfr_gpioout_crgo2: SfrGpiooutCrgo2,
    sfr_gpioout_crgo3: SfrGpiooutCrgo3,
    sfr_gpioout_crgo4: SfrGpiooutCrgo4,
    sfr_gpioout_crgo5: SfrGpiooutCrgo5,
    sfr_gpiooe_crgoe0: SfrGpiooeCrgoe0,
    sfr_gpiooe_crgoe1: SfrGpiooeCrgoe1,
    sfr_gpiooe_crgoe2: SfrGpiooeCrgoe2,
    sfr_gpiooe_crgoe3: SfrGpiooeCrgoe3,
    sfr_gpiooe_crgoe4: SfrGpiooeCrgoe4,
    sfr_gpiooe_crgoe5: SfrGpiooeCrgoe5,
    sfr_gpiopu_crgpu0: SfrGpiopuCrgpu0,
    sfr_gpiopu_crgpu1: SfrGpiopuCrgpu1,
    sfr_gpiopu_crgpu2: SfrGpiopuCrgpu2,
    sfr_gpiopu_crgpu3: SfrGpiopuCrgpu3,
    sfr_gpiopu_crgpu4: SfrGpiopuCrgpu4,
    sfr_gpiopu_crgpu5: SfrGpiopuCrgpu5,
    sfr_gpioin_srgi0: SfrGpioinSrgi0,
    sfr_gpioin_srgi1: SfrGpioinSrgi1,
    sfr_gpioin_srgi2: SfrGpioinSrgi2,
    sfr_gpioin_srgi3: SfrGpioinSrgi3,
    sfr_gpioin_srgi4: SfrGpioinSrgi4,
    sfr_gpioin_srgi5: SfrGpioinSrgi5,
    _reserved45: [u8; 0x70],
    sfr_piosel: SfrPiosel,
    _reserved46: [u8; 0x2c],
    sfr_cfg_schm_cr_cfg_schmsel0: SfrCfgSchmCrCfgSchmsel0,
    sfr_cfg_schm_cr_cfg_schmsel1: SfrCfgSchmCrCfgSchmsel1,
    sfr_cfg_schm_cr_cfg_schmsel2: SfrCfgSchmCrCfgSchmsel2,
    sfr_cfg_schm_cr_cfg_schmsel3: SfrCfgSchmCrCfgSchmsel3,
    sfr_cfg_schm_cr_cfg_schmsel4: SfrCfgSchmCrCfgSchmsel4,
    sfr_cfg_schm_cr_cfg_schmsel5: SfrCfgSchmCrCfgSchmsel5,
    sfr_cfg_slew_cr_cfg_slewslow0: SfrCfgSlewCrCfgSlewslow0,
    sfr_cfg_slew_cr_cfg_slewslow1: SfrCfgSlewCrCfgSlewslow1,
    sfr_cfg_slew_cr_cfg_slewslow2: SfrCfgSlewCrCfgSlewslow2,
    sfr_cfg_slew_cr_cfg_slewslow3: SfrCfgSlewCrCfgSlewslow3,
    sfr_cfg_slew_cr_cfg_slewslow4: SfrCfgSlewCrCfgSlewslow4,
    sfr_cfg_slew_cr_cfg_slewslow5: SfrCfgSlewCrCfgSlewslow5,
    sfr_cfg_drvsel_cr_cfg_drvsel0: SfrCfgDrvselCrCfgDrvsel0,
    sfr_cfg_drvsel_cr_cfg_drvsel1: SfrCfgDrvselCrCfgDrvsel1,
    sfr_cfg_drvsel_cr_cfg_drvsel2: SfrCfgDrvselCrCfgDrvsel2,
    sfr_cfg_drvsel_cr_cfg_drvsel3: SfrCfgDrvselCrCfgDrvsel3,
    sfr_cfg_drvsel_cr_cfg_drvsel4: SfrCfgDrvselCrCfgDrvsel4,
    sfr_cfg_drvsel_cr_cfg_drvsel5: SfrCfgDrvselCrCfgDrvsel5,
}
impl RegisterBlock {
    #[doc = "0x00 - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel0(&self) -> &SfrAfselCrafsel0 {
        &self.sfr_afsel_crafsel0
    }
    #[doc = "0x04 - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel1(&self) -> &SfrAfselCrafsel1 {
        &self.sfr_afsel_crafsel1
    }
    #[doc = "0x08 - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel2(&self) -> &SfrAfselCrafsel2 {
        &self.sfr_afsel_crafsel2
    }
    #[doc = "0x0c - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel3(&self) -> &SfrAfselCrafsel3 {
        &self.sfr_afsel_crafsel3
    }
    #[doc = "0x10 - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel4(&self) -> &SfrAfselCrafsel4 {
        &self.sfr_afsel_crafsel4
    }
    #[doc = "0x14 - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel5(&self) -> &SfrAfselCrafsel5 {
        &self.sfr_afsel_crafsel5
    }
    #[doc = "0x18 - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel6(&self) -> &SfrAfselCrafsel6 {
        &self.sfr_afsel_crafsel6
    }
    #[doc = "0x1c - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel7(&self) -> &SfrAfselCrafsel7 {
        &self.sfr_afsel_crafsel7
    }
    #[doc = "0x20 - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel8(&self) -> &SfrAfselCrafsel8 {
        &self.sfr_afsel_crafsel8
    }
    #[doc = "0x24 - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel9(&self) -> &SfrAfselCrafsel9 {
        &self.sfr_afsel_crafsel9
    }
    #[doc = "0x28 - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel10(&self) -> &SfrAfselCrafsel10 {
        &self.sfr_afsel_crafsel10
    }
    #[doc = "0x2c - See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_afsel_crafsel11(&self) -> &SfrAfselCrafsel11 {
        &self.sfr_afsel_crafsel11
    }
    #[doc = "0x100 - See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_intcr_crint0(&self) -> &SfrIntcrCrint0 {
        &self.sfr_intcr_crint0
    }
    #[doc = "0x104 - See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_intcr_crint1(&self) -> &SfrIntcrCrint1 {
        &self.sfr_intcr_crint1
    }
    #[doc = "0x108 - See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_intcr_crint2(&self) -> &SfrIntcrCrint2 {
        &self.sfr_intcr_crint2
    }
    #[doc = "0x10c - See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_intcr_crint3(&self) -> &SfrIntcrCrint3 {
        &self.sfr_intcr_crint3
    }
    #[doc = "0x110 - See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_intcr_crint4(&self) -> &SfrIntcrCrint4 {
        &self.sfr_intcr_crint4
    }
    #[doc = "0x114 - See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_intcr_crint5(&self) -> &SfrIntcrCrint5 {
        &self.sfr_intcr_crint5
    }
    #[doc = "0x118 - See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_intcr_crint6(&self) -> &SfrIntcrCrint6 {
        &self.sfr_intcr_crint6
    }
    #[doc = "0x11c - See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_intcr_crint7(&self) -> &SfrIntcrCrint7 {
        &self.sfr_intcr_crint7
    }
    #[doc = "0x120 - See `iox.sv#L128 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L128>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_intfr(&self) -> &SfrIntfr {
        &self.sfr_intfr
    }
    #[doc = "0x130 - See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioout_crgo0(&self) -> &SfrGpiooutCrgo0 {
        &self.sfr_gpioout_crgo0
    }
    #[doc = "0x134 - See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioout_crgo1(&self) -> &SfrGpiooutCrgo1 {
        &self.sfr_gpioout_crgo1
    }
    #[doc = "0x138 - See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioout_crgo2(&self) -> &SfrGpiooutCrgo2 {
        &self.sfr_gpioout_crgo2
    }
    #[doc = "0x13c - See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioout_crgo3(&self) -> &SfrGpiooutCrgo3 {
        &self.sfr_gpioout_crgo3
    }
    #[doc = "0x140 - See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioout_crgo4(&self) -> &SfrGpiooutCrgo4 {
        &self.sfr_gpioout_crgo4
    }
    #[doc = "0x144 - See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioout_crgo5(&self) -> &SfrGpiooutCrgo5 {
        &self.sfr_gpioout_crgo5
    }
    #[doc = "0x148 - See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiooe_crgoe0(&self) -> &SfrGpiooeCrgoe0 {
        &self.sfr_gpiooe_crgoe0
    }
    #[doc = "0x14c - See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiooe_crgoe1(&self) -> &SfrGpiooeCrgoe1 {
        &self.sfr_gpiooe_crgoe1
    }
    #[doc = "0x150 - See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiooe_crgoe2(&self) -> &SfrGpiooeCrgoe2 {
        &self.sfr_gpiooe_crgoe2
    }
    #[doc = "0x154 - See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiooe_crgoe3(&self) -> &SfrGpiooeCrgoe3 {
        &self.sfr_gpiooe_crgoe3
    }
    #[doc = "0x158 - See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiooe_crgoe4(&self) -> &SfrGpiooeCrgoe4 {
        &self.sfr_gpiooe_crgoe4
    }
    #[doc = "0x15c - See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiooe_crgoe5(&self) -> &SfrGpiooeCrgoe5 {
        &self.sfr_gpiooe_crgoe5
    }
    #[doc = "0x160 - See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiopu_crgpu0(&self) -> &SfrGpiopuCrgpu0 {
        &self.sfr_gpiopu_crgpu0
    }
    #[doc = "0x164 - See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiopu_crgpu1(&self) -> &SfrGpiopuCrgpu1 {
        &self.sfr_gpiopu_crgpu1
    }
    #[doc = "0x168 - See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiopu_crgpu2(&self) -> &SfrGpiopuCrgpu2 {
        &self.sfr_gpiopu_crgpu2
    }
    #[doc = "0x16c - See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiopu_crgpu3(&self) -> &SfrGpiopuCrgpu3 {
        &self.sfr_gpiopu_crgpu3
    }
    #[doc = "0x170 - See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiopu_crgpu4(&self) -> &SfrGpiopuCrgpu4 {
        &self.sfr_gpiopu_crgpu4
    }
    #[doc = "0x174 - See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpiopu_crgpu5(&self) -> &SfrGpiopuCrgpu5 {
        &self.sfr_gpiopu_crgpu5
    }
    #[doc = "0x178 - See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioin_srgi0(&self) -> &SfrGpioinSrgi0 {
        &self.sfr_gpioin_srgi0
    }
    #[doc = "0x17c - See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioin_srgi1(&self) -> &SfrGpioinSrgi1 {
        &self.sfr_gpioin_srgi1
    }
    #[doc = "0x180 - See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioin_srgi2(&self) -> &SfrGpioinSrgi2 {
        &self.sfr_gpioin_srgi2
    }
    #[doc = "0x184 - See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioin_srgi3(&self) -> &SfrGpioinSrgi3 {
        &self.sfr_gpioin_srgi3
    }
    #[doc = "0x188 - See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioin_srgi4(&self) -> &SfrGpioinSrgi4 {
        &self.sfr_gpioin_srgi4
    }
    #[doc = "0x18c - See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gpioin_srgi5(&self) -> &SfrGpioinSrgi5 {
        &self.sfr_gpioin_srgi5
    }
    #[doc = "0x200 - See `iox.sv#L249 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L249>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_piosel(&self) -> &SfrPiosel {
        &self.sfr_piosel
    }
    #[doc = "0x230 - See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_schm_cr_cfg_schmsel0(&self) -> &SfrCfgSchmCrCfgSchmsel0 {
        &self.sfr_cfg_schm_cr_cfg_schmsel0
    }
    #[doc = "0x234 - See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_schm_cr_cfg_schmsel1(&self) -> &SfrCfgSchmCrCfgSchmsel1 {
        &self.sfr_cfg_schm_cr_cfg_schmsel1
    }
    #[doc = "0x238 - See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_schm_cr_cfg_schmsel2(&self) -> &SfrCfgSchmCrCfgSchmsel2 {
        &self.sfr_cfg_schm_cr_cfg_schmsel2
    }
    #[doc = "0x23c - See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_schm_cr_cfg_schmsel3(&self) -> &SfrCfgSchmCrCfgSchmsel3 {
        &self.sfr_cfg_schm_cr_cfg_schmsel3
    }
    #[doc = "0x240 - See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_schm_cr_cfg_schmsel4(&self) -> &SfrCfgSchmCrCfgSchmsel4 {
        &self.sfr_cfg_schm_cr_cfg_schmsel4
    }
    #[doc = "0x244 - See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_schm_cr_cfg_schmsel5(&self) -> &SfrCfgSchmCrCfgSchmsel5 {
        &self.sfr_cfg_schm_cr_cfg_schmsel5
    }
    #[doc = "0x248 - See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_slew_cr_cfg_slewslow0(&self) -> &SfrCfgSlewCrCfgSlewslow0 {
        &self.sfr_cfg_slew_cr_cfg_slewslow0
    }
    #[doc = "0x24c - See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_slew_cr_cfg_slewslow1(&self) -> &SfrCfgSlewCrCfgSlewslow1 {
        &self.sfr_cfg_slew_cr_cfg_slewslow1
    }
    #[doc = "0x250 - See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_slew_cr_cfg_slewslow2(&self) -> &SfrCfgSlewCrCfgSlewslow2 {
        &self.sfr_cfg_slew_cr_cfg_slewslow2
    }
    #[doc = "0x254 - See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_slew_cr_cfg_slewslow3(&self) -> &SfrCfgSlewCrCfgSlewslow3 {
        &self.sfr_cfg_slew_cr_cfg_slewslow3
    }
    #[doc = "0x258 - See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_slew_cr_cfg_slewslow4(&self) -> &SfrCfgSlewCrCfgSlewslow4 {
        &self.sfr_cfg_slew_cr_cfg_slewslow4
    }
    #[doc = "0x25c - See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_slew_cr_cfg_slewslow5(&self) -> &SfrCfgSlewCrCfgSlewslow5 {
        &self.sfr_cfg_slew_cr_cfg_slewslow5
    }
    #[doc = "0x260 - See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_drvsel_cr_cfg_drvsel0(&self) -> &SfrCfgDrvselCrCfgDrvsel0 {
        &self.sfr_cfg_drvsel_cr_cfg_drvsel0
    }
    #[doc = "0x264 - See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_drvsel_cr_cfg_drvsel1(&self) -> &SfrCfgDrvselCrCfgDrvsel1 {
        &self.sfr_cfg_drvsel_cr_cfg_drvsel1
    }
    #[doc = "0x268 - See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_drvsel_cr_cfg_drvsel2(&self) -> &SfrCfgDrvselCrCfgDrvsel2 {
        &self.sfr_cfg_drvsel_cr_cfg_drvsel2
    }
    #[doc = "0x26c - See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_drvsel_cr_cfg_drvsel3(&self) -> &SfrCfgDrvselCrCfgDrvsel3 {
        &self.sfr_cfg_drvsel_cr_cfg_drvsel3
    }
    #[doc = "0x270 - See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_drvsel_cr_cfg_drvsel4(&self) -> &SfrCfgDrvselCrCfgDrvsel4 {
        &self.sfr_cfg_drvsel_cr_cfg_drvsel4
    }
    #[doc = "0x274 - See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg_drvsel_cr_cfg_drvsel5(&self) -> &SfrCfgDrvselCrCfgDrvsel5 {
        &self.sfr_cfg_drvsel_cr_cfg_drvsel5
    }
}
#[doc = "SFR_AFSEL_CRAFSEL0 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel0`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL0")]
pub type SfrAfselCrafsel0 = crate::Reg<sfr_afsel_crafsel0::SfrAfselCrafsel0Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel0;
#[doc = "SFR_AFSEL_CRAFSEL1 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel1`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL1")]
pub type SfrAfselCrafsel1 = crate::Reg<sfr_afsel_crafsel1::SfrAfselCrafsel1Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel1;
#[doc = "SFR_AFSEL_CRAFSEL2 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel2`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL2")]
pub type SfrAfselCrafsel2 = crate::Reg<sfr_afsel_crafsel2::SfrAfselCrafsel2Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel2;
#[doc = "SFR_AFSEL_CRAFSEL3 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel3`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL3")]
pub type SfrAfselCrafsel3 = crate::Reg<sfr_afsel_crafsel3::SfrAfselCrafsel3Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel3;
#[doc = "SFR_AFSEL_CRAFSEL4 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel4`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL4")]
pub type SfrAfselCrafsel4 = crate::Reg<sfr_afsel_crafsel4::SfrAfselCrafsel4Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel4;
#[doc = "SFR_AFSEL_CRAFSEL5 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel5`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL5")]
pub type SfrAfselCrafsel5 = crate::Reg<sfr_afsel_crafsel5::SfrAfselCrafsel5Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel5;
#[doc = "SFR_AFSEL_CRAFSEL6 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel6`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL6")]
pub type SfrAfselCrafsel6 = crate::Reg<sfr_afsel_crafsel6::SfrAfselCrafsel6Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel6;
#[doc = "SFR_AFSEL_CRAFSEL7 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel7`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL7")]
pub type SfrAfselCrafsel7 = crate::Reg<sfr_afsel_crafsel7::SfrAfselCrafsel7Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel7;
#[doc = "SFR_AFSEL_CRAFSEL8 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel8`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL8")]
pub type SfrAfselCrafsel8 = crate::Reg<sfr_afsel_crafsel8::SfrAfselCrafsel8Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel8;
#[doc = "SFR_AFSEL_CRAFSEL9 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel9`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL9")]
pub type SfrAfselCrafsel9 = crate::Reg<sfr_afsel_crafsel9::SfrAfselCrafsel9Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel9;
#[doc = "SFR_AFSEL_CRAFSEL10 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel10`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL10")]
pub type SfrAfselCrafsel10 = crate::Reg<sfr_afsel_crafsel10::SfrAfselCrafsel10Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel10;
#[doc = "SFR_AFSEL_CRAFSEL11 (rw) register accessor: See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_afsel_crafsel11`] module"]
#[doc(alias = "SFR_AFSEL_CRAFSEL11")]
pub type SfrAfselCrafsel11 = crate::Reg<sfr_afsel_crafsel11::SfrAfselCrafsel11Spec>;
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)"]
pub mod sfr_afsel_crafsel11;
#[doc = "SFR_INTCR_CRINT0 (rw) register accessor: See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_intcr_crint0`] module"]
#[doc(alias = "SFR_INTCR_CRINT0")]
pub type SfrIntcrCrint0 = crate::Reg<sfr_intcr_crint0::SfrIntcrCrint0Spec>;
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
pub mod sfr_intcr_crint0;
#[doc = "SFR_INTCR_CRINT1 (rw) register accessor: See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_intcr_crint1`] module"]
#[doc(alias = "SFR_INTCR_CRINT1")]
pub type SfrIntcrCrint1 = crate::Reg<sfr_intcr_crint1::SfrIntcrCrint1Spec>;
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
pub mod sfr_intcr_crint1;
#[doc = "SFR_INTCR_CRINT2 (rw) register accessor: See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_intcr_crint2`] module"]
#[doc(alias = "SFR_INTCR_CRINT2")]
pub type SfrIntcrCrint2 = crate::Reg<sfr_intcr_crint2::SfrIntcrCrint2Spec>;
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
pub mod sfr_intcr_crint2;
#[doc = "SFR_INTCR_CRINT3 (rw) register accessor: See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_intcr_crint3`] module"]
#[doc(alias = "SFR_INTCR_CRINT3")]
pub type SfrIntcrCrint3 = crate::Reg<sfr_intcr_crint3::SfrIntcrCrint3Spec>;
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
pub mod sfr_intcr_crint3;
#[doc = "SFR_INTCR_CRINT4 (rw) register accessor: See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_intcr_crint4`] module"]
#[doc(alias = "SFR_INTCR_CRINT4")]
pub type SfrIntcrCrint4 = crate::Reg<sfr_intcr_crint4::SfrIntcrCrint4Spec>;
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
pub mod sfr_intcr_crint4;
#[doc = "SFR_INTCR_CRINT5 (rw) register accessor: See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_intcr_crint5`] module"]
#[doc(alias = "SFR_INTCR_CRINT5")]
pub type SfrIntcrCrint5 = crate::Reg<sfr_intcr_crint5::SfrIntcrCrint5Spec>;
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
pub mod sfr_intcr_crint5;
#[doc = "SFR_INTCR_CRINT6 (rw) register accessor: See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_intcr_crint6`] module"]
#[doc(alias = "SFR_INTCR_CRINT6")]
pub type SfrIntcrCrint6 = crate::Reg<sfr_intcr_crint6::SfrIntcrCrint6Spec>;
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
pub mod sfr_intcr_crint6;
#[doc = "SFR_INTCR_CRINT7 (rw) register accessor: See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_intcr_crint7`] module"]
#[doc(alias = "SFR_INTCR_CRINT7")]
pub type SfrIntcrCrint7 = crate::Reg<sfr_intcr_crint7::SfrIntcrCrint7Spec>;
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)"]
pub mod sfr_intcr_crint7;
#[doc = "SFR_INTFR (rw) register accessor: See `iox.sv#L128 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L128>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_intfr`] module"]
#[doc(alias = "SFR_INTFR")]
pub type SfrIntfr = crate::Reg<sfr_intfr::SfrIntfrSpec>;
#[doc = "See `iox.sv#L128 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L128>`__ (line numbers are approximate)"]
pub mod sfr_intfr;
#[doc = "SFR_GPIOOUT_CRGO0 (rw) register accessor: See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioout_crgo0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioout_crgo0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioout_crgo0`] module"]
#[doc(alias = "SFR_GPIOOUT_CRGO0")]
pub type SfrGpiooutCrgo0 = crate::Reg<sfr_gpioout_crgo0::SfrGpiooutCrgo0Spec>;
#[doc = "See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_gpioout_crgo0;
#[doc = "SFR_GPIOOUT_CRGO1 (rw) register accessor: See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioout_crgo1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioout_crgo1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioout_crgo1`] module"]
#[doc(alias = "SFR_GPIOOUT_CRGO1")]
pub type SfrGpiooutCrgo1 = crate::Reg<sfr_gpioout_crgo1::SfrGpiooutCrgo1Spec>;
#[doc = "See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_gpioout_crgo1;
#[doc = "SFR_GPIOOUT_CRGO2 (rw) register accessor: See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioout_crgo2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioout_crgo2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioout_crgo2`] module"]
#[doc(alias = "SFR_GPIOOUT_CRGO2")]
pub type SfrGpiooutCrgo2 = crate::Reg<sfr_gpioout_crgo2::SfrGpiooutCrgo2Spec>;
#[doc = "See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_gpioout_crgo2;
#[doc = "SFR_GPIOOUT_CRGO3 (rw) register accessor: See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioout_crgo3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioout_crgo3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioout_crgo3`] module"]
#[doc(alias = "SFR_GPIOOUT_CRGO3")]
pub type SfrGpiooutCrgo3 = crate::Reg<sfr_gpioout_crgo3::SfrGpiooutCrgo3Spec>;
#[doc = "See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_gpioout_crgo3;
#[doc = "SFR_GPIOOUT_CRGO4 (rw) register accessor: See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioout_crgo4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioout_crgo4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioout_crgo4`] module"]
#[doc(alias = "SFR_GPIOOUT_CRGO4")]
pub type SfrGpiooutCrgo4 = crate::Reg<sfr_gpioout_crgo4::SfrGpiooutCrgo4Spec>;
#[doc = "See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_gpioout_crgo4;
#[doc = "SFR_GPIOOUT_CRGO5 (rw) register accessor: See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioout_crgo5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioout_crgo5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioout_crgo5`] module"]
#[doc(alias = "SFR_GPIOOUT_CRGO5")]
pub type SfrGpiooutCrgo5 = crate::Reg<sfr_gpioout_crgo5::SfrGpiooutCrgo5Spec>;
#[doc = "See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_gpioout_crgo5;
#[doc = "SFR_GPIOOE_CRGOE0 (rw) register accessor: See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiooe_crgoe0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiooe_crgoe0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiooe_crgoe0`] module"]
#[doc(alias = "SFR_GPIOOE_CRGOE0")]
pub type SfrGpiooeCrgoe0 = crate::Reg<sfr_gpiooe_crgoe0::SfrGpiooeCrgoe0Spec>;
#[doc = "See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_gpiooe_crgoe0;
#[doc = "SFR_GPIOOE_CRGOE1 (rw) register accessor: See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiooe_crgoe1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiooe_crgoe1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiooe_crgoe1`] module"]
#[doc(alias = "SFR_GPIOOE_CRGOE1")]
pub type SfrGpiooeCrgoe1 = crate::Reg<sfr_gpiooe_crgoe1::SfrGpiooeCrgoe1Spec>;
#[doc = "See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_gpiooe_crgoe1;
#[doc = "SFR_GPIOOE_CRGOE2 (rw) register accessor: See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiooe_crgoe2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiooe_crgoe2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiooe_crgoe2`] module"]
#[doc(alias = "SFR_GPIOOE_CRGOE2")]
pub type SfrGpiooeCrgoe2 = crate::Reg<sfr_gpiooe_crgoe2::SfrGpiooeCrgoe2Spec>;
#[doc = "See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_gpiooe_crgoe2;
#[doc = "SFR_GPIOOE_CRGOE3 (rw) register accessor: See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiooe_crgoe3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiooe_crgoe3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiooe_crgoe3`] module"]
#[doc(alias = "SFR_GPIOOE_CRGOE3")]
pub type SfrGpiooeCrgoe3 = crate::Reg<sfr_gpiooe_crgoe3::SfrGpiooeCrgoe3Spec>;
#[doc = "See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_gpiooe_crgoe3;
#[doc = "SFR_GPIOOE_CRGOE4 (rw) register accessor: See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiooe_crgoe4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiooe_crgoe4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiooe_crgoe4`] module"]
#[doc(alias = "SFR_GPIOOE_CRGOE4")]
pub type SfrGpiooeCrgoe4 = crate::Reg<sfr_gpiooe_crgoe4::SfrGpiooeCrgoe4Spec>;
#[doc = "See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_gpiooe_crgoe4;
#[doc = "SFR_GPIOOE_CRGOE5 (rw) register accessor: See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiooe_crgoe5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiooe_crgoe5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiooe_crgoe5`] module"]
#[doc(alias = "SFR_GPIOOE_CRGOE5")]
pub type SfrGpiooeCrgoe5 = crate::Reg<sfr_gpiooe_crgoe5::SfrGpiooeCrgoe5Spec>;
#[doc = "See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_gpiooe_crgoe5;
#[doc = "SFR_GPIOPU_CRGPU0 (rw) register accessor: See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiopu_crgpu0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiopu_crgpu0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiopu_crgpu0`] module"]
#[doc(alias = "SFR_GPIOPU_CRGPU0")]
pub type SfrGpiopuCrgpu0 = crate::Reg<sfr_gpiopu_crgpu0::SfrGpiopuCrgpu0Spec>;
#[doc = "See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
pub mod sfr_gpiopu_crgpu0;
#[doc = "SFR_GPIOPU_CRGPU1 (rw) register accessor: See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiopu_crgpu1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiopu_crgpu1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiopu_crgpu1`] module"]
#[doc(alias = "SFR_GPIOPU_CRGPU1")]
pub type SfrGpiopuCrgpu1 = crate::Reg<sfr_gpiopu_crgpu1::SfrGpiopuCrgpu1Spec>;
#[doc = "See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
pub mod sfr_gpiopu_crgpu1;
#[doc = "SFR_GPIOPU_CRGPU2 (rw) register accessor: See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiopu_crgpu2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiopu_crgpu2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiopu_crgpu2`] module"]
#[doc(alias = "SFR_GPIOPU_CRGPU2")]
pub type SfrGpiopuCrgpu2 = crate::Reg<sfr_gpiopu_crgpu2::SfrGpiopuCrgpu2Spec>;
#[doc = "See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
pub mod sfr_gpiopu_crgpu2;
#[doc = "SFR_GPIOPU_CRGPU3 (rw) register accessor: See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiopu_crgpu3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiopu_crgpu3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiopu_crgpu3`] module"]
#[doc(alias = "SFR_GPIOPU_CRGPU3")]
pub type SfrGpiopuCrgpu3 = crate::Reg<sfr_gpiopu_crgpu3::SfrGpiopuCrgpu3Spec>;
#[doc = "See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
pub mod sfr_gpiopu_crgpu3;
#[doc = "SFR_GPIOPU_CRGPU4 (rw) register accessor: See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiopu_crgpu4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiopu_crgpu4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiopu_crgpu4`] module"]
#[doc(alias = "SFR_GPIOPU_CRGPU4")]
pub type SfrGpiopuCrgpu4 = crate::Reg<sfr_gpiopu_crgpu4::SfrGpiopuCrgpu4Spec>;
#[doc = "See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
pub mod sfr_gpiopu_crgpu4;
#[doc = "SFR_GPIOPU_CRGPU5 (rw) register accessor: See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiopu_crgpu5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiopu_crgpu5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpiopu_crgpu5`] module"]
#[doc(alias = "SFR_GPIOPU_CRGPU5")]
pub type SfrGpiopuCrgpu5 = crate::Reg<sfr_gpiopu_crgpu5::SfrGpiopuCrgpu5Spec>;
#[doc = "See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)"]
pub mod sfr_gpiopu_crgpu5;
#[doc = "SFR_GPIOIN_SRGI0 (rw) register accessor: See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioin_srgi0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioin_srgi0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioin_srgi0`] module"]
#[doc(alias = "SFR_GPIOIN_SRGI0")]
pub type SfrGpioinSrgi0 = crate::Reg<sfr_gpioin_srgi0::SfrGpioinSrgi0Spec>;
#[doc = "See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
pub mod sfr_gpioin_srgi0;
#[doc = "SFR_GPIOIN_SRGI1 (rw) register accessor: See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioin_srgi1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioin_srgi1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioin_srgi1`] module"]
#[doc(alias = "SFR_GPIOIN_SRGI1")]
pub type SfrGpioinSrgi1 = crate::Reg<sfr_gpioin_srgi1::SfrGpioinSrgi1Spec>;
#[doc = "See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
pub mod sfr_gpioin_srgi1;
#[doc = "SFR_GPIOIN_SRGI2 (rw) register accessor: See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioin_srgi2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioin_srgi2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioin_srgi2`] module"]
#[doc(alias = "SFR_GPIOIN_SRGI2")]
pub type SfrGpioinSrgi2 = crate::Reg<sfr_gpioin_srgi2::SfrGpioinSrgi2Spec>;
#[doc = "See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
pub mod sfr_gpioin_srgi2;
#[doc = "SFR_GPIOIN_SRGI3 (rw) register accessor: See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioin_srgi3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioin_srgi3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioin_srgi3`] module"]
#[doc(alias = "SFR_GPIOIN_SRGI3")]
pub type SfrGpioinSrgi3 = crate::Reg<sfr_gpioin_srgi3::SfrGpioinSrgi3Spec>;
#[doc = "See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
pub mod sfr_gpioin_srgi3;
#[doc = "SFR_GPIOIN_SRGI4 (rw) register accessor: See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioin_srgi4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioin_srgi4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioin_srgi4`] module"]
#[doc(alias = "SFR_GPIOIN_SRGI4")]
pub type SfrGpioinSrgi4 = crate::Reg<sfr_gpioin_srgi4::SfrGpioinSrgi4Spec>;
#[doc = "See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
pub mod sfr_gpioin_srgi4;
#[doc = "SFR_GPIOIN_SRGI5 (rw) register accessor: See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioin_srgi5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioin_srgi5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gpioin_srgi5`] module"]
#[doc(alias = "SFR_GPIOIN_SRGI5")]
pub type SfrGpioinSrgi5 = crate::Reg<sfr_gpioin_srgi5::SfrGpioinSrgi5Spec>;
#[doc = "See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)"]
pub mod sfr_gpioin_srgi5;
#[doc = "SFR_PIOSEL (rw) register accessor: See `iox.sv#L249 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L249>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_piosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_piosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_piosel`] module"]
#[doc(alias = "SFR_PIOSEL")]
pub type SfrPiosel = crate::Reg<sfr_piosel::SfrPioselSpec>;
#[doc = "See `iox.sv#L249 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L249>`__ (line numbers are approximate)"]
pub mod sfr_piosel;
#[doc = "SFR_CFG_SCHM_CR_CFG_SCHMSEL0 (rw) register accessor: See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_schm_cr_cfg_schmsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_schm_cr_cfg_schmsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_schm_cr_cfg_schmsel0`] module"]
#[doc(alias = "SFR_CFG_SCHM_CR_CFG_SCHMSEL0")]
pub type SfrCfgSchmCrCfgSchmsel0 =
    crate::Reg<sfr_cfg_schm_cr_cfg_schmsel0::SfrCfgSchmCrCfgSchmsel0Spec>;
#[doc = "See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
pub mod sfr_cfg_schm_cr_cfg_schmsel0;
#[doc = "SFR_CFG_SCHM_CR_CFG_SCHMSEL1 (rw) register accessor: See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_schm_cr_cfg_schmsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_schm_cr_cfg_schmsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_schm_cr_cfg_schmsel1`] module"]
#[doc(alias = "SFR_CFG_SCHM_CR_CFG_SCHMSEL1")]
pub type SfrCfgSchmCrCfgSchmsel1 =
    crate::Reg<sfr_cfg_schm_cr_cfg_schmsel1::SfrCfgSchmCrCfgSchmsel1Spec>;
#[doc = "See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
pub mod sfr_cfg_schm_cr_cfg_schmsel1;
#[doc = "SFR_CFG_SCHM_CR_CFG_SCHMSEL2 (rw) register accessor: See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_schm_cr_cfg_schmsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_schm_cr_cfg_schmsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_schm_cr_cfg_schmsel2`] module"]
#[doc(alias = "SFR_CFG_SCHM_CR_CFG_SCHMSEL2")]
pub type SfrCfgSchmCrCfgSchmsel2 =
    crate::Reg<sfr_cfg_schm_cr_cfg_schmsel2::SfrCfgSchmCrCfgSchmsel2Spec>;
#[doc = "See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
pub mod sfr_cfg_schm_cr_cfg_schmsel2;
#[doc = "SFR_CFG_SCHM_CR_CFG_SCHMSEL3 (rw) register accessor: See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_schm_cr_cfg_schmsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_schm_cr_cfg_schmsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_schm_cr_cfg_schmsel3`] module"]
#[doc(alias = "SFR_CFG_SCHM_CR_CFG_SCHMSEL3")]
pub type SfrCfgSchmCrCfgSchmsel3 =
    crate::Reg<sfr_cfg_schm_cr_cfg_schmsel3::SfrCfgSchmCrCfgSchmsel3Spec>;
#[doc = "See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
pub mod sfr_cfg_schm_cr_cfg_schmsel3;
#[doc = "SFR_CFG_SCHM_CR_CFG_SCHMSEL4 (rw) register accessor: See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_schm_cr_cfg_schmsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_schm_cr_cfg_schmsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_schm_cr_cfg_schmsel4`] module"]
#[doc(alias = "SFR_CFG_SCHM_CR_CFG_SCHMSEL4")]
pub type SfrCfgSchmCrCfgSchmsel4 =
    crate::Reg<sfr_cfg_schm_cr_cfg_schmsel4::SfrCfgSchmCrCfgSchmsel4Spec>;
#[doc = "See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
pub mod sfr_cfg_schm_cr_cfg_schmsel4;
#[doc = "SFR_CFG_SCHM_CR_CFG_SCHMSEL5 (rw) register accessor: See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_schm_cr_cfg_schmsel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_schm_cr_cfg_schmsel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_schm_cr_cfg_schmsel5`] module"]
#[doc(alias = "SFR_CFG_SCHM_CR_CFG_SCHMSEL5")]
pub type SfrCfgSchmCrCfgSchmsel5 =
    crate::Reg<sfr_cfg_schm_cr_cfg_schmsel5::SfrCfgSchmCrCfgSchmsel5Spec>;
#[doc = "See `iox.sv#L225 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L225>`__ (line numbers are approximate)"]
pub mod sfr_cfg_schm_cr_cfg_schmsel5;
#[doc = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW0 (rw) register accessor: See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_slew_cr_cfg_slewslow0`] module"]
#[doc(alias = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW0")]
pub type SfrCfgSlewCrCfgSlewslow0 =
    crate::Reg<sfr_cfg_slew_cr_cfg_slewslow0::SfrCfgSlewCrCfgSlewslow0Spec>;
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
pub mod sfr_cfg_slew_cr_cfg_slewslow0;
#[doc = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW1 (rw) register accessor: See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_slew_cr_cfg_slewslow1`] module"]
#[doc(alias = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW1")]
pub type SfrCfgSlewCrCfgSlewslow1 =
    crate::Reg<sfr_cfg_slew_cr_cfg_slewslow1::SfrCfgSlewCrCfgSlewslow1Spec>;
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
pub mod sfr_cfg_slew_cr_cfg_slewslow1;
#[doc = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW2 (rw) register accessor: See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_slew_cr_cfg_slewslow2`] module"]
#[doc(alias = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW2")]
pub type SfrCfgSlewCrCfgSlewslow2 =
    crate::Reg<sfr_cfg_slew_cr_cfg_slewslow2::SfrCfgSlewCrCfgSlewslow2Spec>;
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
pub mod sfr_cfg_slew_cr_cfg_slewslow2;
#[doc = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW3 (rw) register accessor: See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_slew_cr_cfg_slewslow3`] module"]
#[doc(alias = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW3")]
pub type SfrCfgSlewCrCfgSlewslow3 =
    crate::Reg<sfr_cfg_slew_cr_cfg_slewslow3::SfrCfgSlewCrCfgSlewslow3Spec>;
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
pub mod sfr_cfg_slew_cr_cfg_slewslow3;
#[doc = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW4 (rw) register accessor: See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_slew_cr_cfg_slewslow4`] module"]
#[doc(alias = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW4")]
pub type SfrCfgSlewCrCfgSlewslow4 =
    crate::Reg<sfr_cfg_slew_cr_cfg_slewslow4::SfrCfgSlewCrCfgSlewslow4Spec>;
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
pub mod sfr_cfg_slew_cr_cfg_slewslow4;
#[doc = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW5 (rw) register accessor: See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_slew_cr_cfg_slewslow5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_slew_cr_cfg_slewslow5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_slew_cr_cfg_slewslow5`] module"]
#[doc(alias = "SFR_CFG_SLEW_CR_CFG_SLEWSLOW5")]
pub type SfrCfgSlewCrCfgSlewslow5 =
    crate::Reg<sfr_cfg_slew_cr_cfg_slewslow5::SfrCfgSlewCrCfgSlewslow5Spec>;
#[doc = "See `iox.sv#L226 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L226>`__ (line numbers are approximate)"]
pub mod sfr_cfg_slew_cr_cfg_slewslow5;
#[doc = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL0 (rw) register accessor: See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_drvsel_cr_cfg_drvsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_drvsel_cr_cfg_drvsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_drvsel_cr_cfg_drvsel0`] module"]
#[doc(alias = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL0")]
pub type SfrCfgDrvselCrCfgDrvsel0 =
    crate::Reg<sfr_cfg_drvsel_cr_cfg_drvsel0::SfrCfgDrvselCrCfgDrvsel0Spec>;
#[doc = "See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
pub mod sfr_cfg_drvsel_cr_cfg_drvsel0;
#[doc = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL1 (rw) register accessor: See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_drvsel_cr_cfg_drvsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_drvsel_cr_cfg_drvsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_drvsel_cr_cfg_drvsel1`] module"]
#[doc(alias = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL1")]
pub type SfrCfgDrvselCrCfgDrvsel1 =
    crate::Reg<sfr_cfg_drvsel_cr_cfg_drvsel1::SfrCfgDrvselCrCfgDrvsel1Spec>;
#[doc = "See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
pub mod sfr_cfg_drvsel_cr_cfg_drvsel1;
#[doc = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL2 (rw) register accessor: See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_drvsel_cr_cfg_drvsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_drvsel_cr_cfg_drvsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_drvsel_cr_cfg_drvsel2`] module"]
#[doc(alias = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL2")]
pub type SfrCfgDrvselCrCfgDrvsel2 =
    crate::Reg<sfr_cfg_drvsel_cr_cfg_drvsel2::SfrCfgDrvselCrCfgDrvsel2Spec>;
#[doc = "See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
pub mod sfr_cfg_drvsel_cr_cfg_drvsel2;
#[doc = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL3 (rw) register accessor: See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_drvsel_cr_cfg_drvsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_drvsel_cr_cfg_drvsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_drvsel_cr_cfg_drvsel3`] module"]
#[doc(alias = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL3")]
pub type SfrCfgDrvselCrCfgDrvsel3 =
    crate::Reg<sfr_cfg_drvsel_cr_cfg_drvsel3::SfrCfgDrvselCrCfgDrvsel3Spec>;
#[doc = "See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
pub mod sfr_cfg_drvsel_cr_cfg_drvsel3;
#[doc = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL4 (rw) register accessor: See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_drvsel_cr_cfg_drvsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_drvsel_cr_cfg_drvsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_drvsel_cr_cfg_drvsel4`] module"]
#[doc(alias = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL4")]
pub type SfrCfgDrvselCrCfgDrvsel4 =
    crate::Reg<sfr_cfg_drvsel_cr_cfg_drvsel4::SfrCfgDrvselCrCfgDrvsel4Spec>;
#[doc = "See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
pub mod sfr_cfg_drvsel_cr_cfg_drvsel4;
#[doc = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL5 (rw) register accessor: See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg_drvsel_cr_cfg_drvsel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg_drvsel_cr_cfg_drvsel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg_drvsel_cr_cfg_drvsel5`] module"]
#[doc(alias = "SFR_CFG_DRVSEL_CR_CFG_DRVSEL5")]
pub type SfrCfgDrvselCrCfgDrvsel5 =
    crate::Reg<sfr_cfg_drvsel_cr_cfg_drvsel5::SfrCfgDrvselCrCfgDrvsel5Spec>;
#[doc = "See `iox.sv#L227 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L227>`__ (line numbers are approximate)"]
pub mod sfr_cfg_drvsel_cr_cfg_drvsel5;
