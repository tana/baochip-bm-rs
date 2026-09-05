#[doc = "Register `SFR_ICH_WPSTART` reader"]
pub type R = crate::R<SfrIchWpstartSpec>;
#[doc = "Register `SFR_ICH_WPSTART` writer"]
pub type W = crate::W<SfrIchWpstartSpec>;
#[doc = "Field `ichcr_wpstart` reader - ichcr_wpstart read/write control register"]
pub type IchcrWpstartR = crate::FieldReader<u16>;
#[doc = "Field `ichcr_wpstart` writer - ichcr_wpstart read/write control register"]
pub type IchcrWpstartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ichcr_wpstart read/write control register"]
    #[inline(always)]
    pub fn ichcr_wpstart(&self) -> IchcrWpstartR {
        IchcrWpstartR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ichcr_wpstart read/write control register"]
    #[inline(always)]
    pub fn ichcr_wpstart(&mut self) -> IchcrWpstartW<'_, SfrIchWpstartSpec> {
        IchcrWpstartW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L114>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_wpstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_wpstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIchWpstartSpec;
impl crate::RegisterSpec for SfrIchWpstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ich_wpstart::R`](R) reader structure"]
impl crate::Readable for SfrIchWpstartSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ich_wpstart::W`](W) writer structure"]
impl crate::Writable for SfrIchWpstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ICH_WPSTART to value 0"]
impl crate::Resettable for SfrIchWpstartSpec {}
