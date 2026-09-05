#[doc = "Register `SFR_ICH_TRANSIZE` reader"]
pub type R = crate::R<SfrIchTransizeSpec>;
#[doc = "Register `SFR_ICH_TRANSIZE` writer"]
pub type W = crate::W<SfrIchTransizeSpec>;
#[doc = "Field `ichcr_transize` reader - ichcr_transize read/write control register"]
pub type IchcrTransizeR = crate::FieldReader<u16>;
#[doc = "Field `ichcr_transize` writer - ichcr_transize read/write control register"]
pub type IchcrTransizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ichcr_transize read/write control register"]
    #[inline(always)]
    pub fn ichcr_transize(&self) -> IchcrTransizeR {
        IchcrTransizeR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ichcr_transize read/write control register"]
    #[inline(always)]
    pub fn ichcr_transize(&mut self) -> IchcrTransizeW<'_, SfrIchTransizeSpec> {
        IchcrTransizeW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L115 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L115>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_transize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_transize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIchTransizeSpec;
impl crate::RegisterSpec for SfrIchTransizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ich_transize::R`](R) reader structure"]
impl crate::Readable for SfrIchTransizeSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ich_transize::W`](W) writer structure"]
impl crate::Writable for SfrIchTransizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ICH_TRANSIZE to value 0"]
impl crate::Resettable for SfrIchTransizeSpec {}
