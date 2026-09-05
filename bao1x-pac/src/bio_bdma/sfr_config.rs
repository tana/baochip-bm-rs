#[doc = "Register `SFR_CONFIG` reader"]
pub type R = crate::R<SfrConfigSpec>;
#[doc = "Register `SFR_CONFIG` writer"]
pub type W = crate::W<SfrConfigSpec>;
#[doc = "Field `snap_output_to_which` reader - snap_output_to_which read/write control register"]
pub type SnapOutputToWhichR = crate::FieldReader;
#[doc = "Field `snap_output_to_which` writer - snap_output_to_which read/write control register"]
pub type SnapOutputToWhichW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `snap_output_to_quantum` reader - snap_output_to_quantum read/write control register"]
pub type SnapOutputToQuantumR = crate::BitReader;
#[doc = "Field `snap_output_to_quantum` writer - snap_output_to_quantum read/write control register"]
pub type SnapOutputToQuantumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `snap_input_to_which` reader - snap_input_to_which read/write control register"]
pub type SnapInputToWhichR = crate::FieldReader;
#[doc = "Field `snap_input_to_which` writer - snap_input_to_which read/write control register"]
pub type SnapInputToWhichW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `snap_input_to_quantum` reader - snap_input_to_quantum read/write control register"]
pub type SnapInputToQuantumR = crate::BitReader;
#[doc = "Field `snap_input_to_quantum` writer - snap_input_to_quantum read/write control register"]
pub type SnapInputToQuantumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disable_filter_peri` reader - disable_filter_peri read/write control register"]
pub type DisableFilterPeriR = crate::BitReader;
#[doc = "Field `disable_filter_peri` writer - disable_filter_peri read/write control register"]
pub type DisableFilterPeriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disable_filter_mem` reader - disable_filter_mem read/write control register"]
pub type DisableFilterMemR = crate::BitReader;
#[doc = "Field `disable_filter_mem` writer - disable_filter_mem read/write control register"]
pub type DisableFilterMemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clocking_mode` reader - clocking_mode read/write control register"]
pub type ClockingModeR = crate::FieldReader;
#[doc = "Field `clocking_mode` writer - clocking_mode read/write control register"]
pub type ClockingModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - snap_output_to_which read/write control register"]
    #[inline(always)]
    pub fn snap_output_to_which(&self) -> SnapOutputToWhichR {
        SnapOutputToWhichR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - snap_output_to_quantum read/write control register"]
    #[inline(always)]
    pub fn snap_output_to_quantum(&self) -> SnapOutputToQuantumR {
        SnapOutputToQuantumR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - snap_input_to_which read/write control register"]
    #[inline(always)]
    pub fn snap_input_to_which(&self) -> SnapInputToWhichR {
        SnapInputToWhichR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - snap_input_to_quantum read/write control register"]
    #[inline(always)]
    pub fn snap_input_to_quantum(&self) -> SnapInputToQuantumR {
        SnapInputToQuantumR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - disable_filter_peri read/write control register"]
    #[inline(always)]
    pub fn disable_filter_peri(&self) -> DisableFilterPeriR {
        DisableFilterPeriR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - disable_filter_mem read/write control register"]
    #[inline(always)]
    pub fn disable_filter_mem(&self) -> DisableFilterMemR {
        DisableFilterMemR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - clocking_mode read/write control register"]
    #[inline(always)]
    pub fn clocking_mode(&self) -> ClockingModeR {
        ClockingModeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - snap_output_to_which read/write control register"]
    #[inline(always)]
    pub fn snap_output_to_which(&mut self) -> SnapOutputToWhichW<'_, SfrConfigSpec> {
        SnapOutputToWhichW::new(self, 0)
    }
    #[doc = "Bit 2 - snap_output_to_quantum read/write control register"]
    #[inline(always)]
    pub fn snap_output_to_quantum(&mut self) -> SnapOutputToQuantumW<'_, SfrConfigSpec> {
        SnapOutputToQuantumW::new(self, 2)
    }
    #[doc = "Bits 3:4 - snap_input_to_which read/write control register"]
    #[inline(always)]
    pub fn snap_input_to_which(&mut self) -> SnapInputToWhichW<'_, SfrConfigSpec> {
        SnapInputToWhichW::new(self, 3)
    }
    #[doc = "Bit 5 - snap_input_to_quantum read/write control register"]
    #[inline(always)]
    pub fn snap_input_to_quantum(&mut self) -> SnapInputToQuantumW<'_, SfrConfigSpec> {
        SnapInputToQuantumW::new(self, 5)
    }
    #[doc = "Bit 6 - disable_filter_peri read/write control register"]
    #[inline(always)]
    pub fn disable_filter_peri(&mut self) -> DisableFilterPeriW<'_, SfrConfigSpec> {
        DisableFilterPeriW::new(self, 6)
    }
    #[doc = "Bit 7 - disable_filter_mem read/write control register"]
    #[inline(always)]
    pub fn disable_filter_mem(&mut self) -> DisableFilterMemW<'_, SfrConfigSpec> {
        DisableFilterMemW::new(self, 7)
    }
    #[doc = "Bits 8:9 - clocking_mode read/write control register"]
    #[inline(always)]
    pub fn clocking_mode(&mut self) -> ClockingModeW<'_, SfrConfigSpec> {
        ClockingModeW::new(self, 8)
    }
}
#[doc = "See `bio_bdma.sv#L490 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L490>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrConfigSpec;
impl crate::RegisterSpec for SfrConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_config::R`](R) reader structure"]
impl crate::Readable for SfrConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_config::W`](W) writer structure"]
impl crate::Writable for SfrConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CONFIG to value 0"]
impl crate::Resettable for SfrConfigSpec {}
