#[doc = "Register `SFR_CTRL` reader"]
pub type R = crate::R<SfrCtrlSpec>;
#[doc = "Register `SFR_CTRL` writer"]
pub type W = crate::W<SfrCtrlSpec>;
#[doc = "Field `en` reader - en read/write control register"]
pub type EnR = crate::FieldReader;
#[doc = "Field `en` writer - en read/write control register"]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `restart` reader - restart read/write control register"]
pub type RestartR = crate::FieldReader;
#[doc = "Field `restart` writer - restart read/write control register"]
pub type RestartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `clkdiv_restart` reader - clkdiv_restart read/write control register"]
pub type ClkdivRestartR = crate::FieldReader;
#[doc = "Field `clkdiv_restart` writer - clkdiv_restart read/write control register"]
pub type ClkdivRestartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - en read/write control register"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - restart read/write control register"]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - clkdiv_restart read/write control register"]
    #[inline(always)]
    pub fn clkdiv_restart(&self) -> ClkdivRestartR {
        ClkdivRestartR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - en read/write control register"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, SfrCtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - restart read/write control register"]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, SfrCtrlSpec> {
        RestartW::new(self, 4)
    }
    #[doc = "Bits 8:11 - clkdiv_restart read/write control register"]
    #[inline(always)]
    pub fn clkdiv_restart(&mut self) -> ClkdivRestartW<'_, SfrCtrlSpec> {
        ClkdivRestartW::new(self, 8)
    }
}
#[doc = "See `bio_bdma.sv#L488 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L488>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCtrlSpec;
impl crate::RegisterSpec for SfrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ctrl::R`](R) reader structure"]
impl crate::Readable for SfrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ctrl::W`](W) writer structure"]
impl crate::Writable for SfrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CTRL to value 0"]
impl crate::Resettable for SfrCtrlSpec {}
