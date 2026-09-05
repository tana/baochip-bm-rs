#[doc = "Register `SFR_OPTMASK` reader"]
pub type R = crate::R<SfrOptmaskSpec>;
#[doc = "Register `SFR_OPTMASK` writer"]
pub type W = crate::W<SfrOptmaskSpec>;
#[doc = "Field `sfr_optmask` reader - sfr_optmask read/write control register"]
pub type SfrOptmaskR = crate::FieldReader<u16>;
#[doc = "Field `sfr_optmask` writer - sfr_optmask read/write control register"]
pub type SfrOptmaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_optmask read/write control register"]
    #[inline(always)]
    pub fn sfr_optmask(&self) -> SfrOptmaskR {
        SfrOptmaskR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_optmask read/write control register"]
    #[inline(always)]
    pub fn sfr_optmask(&mut self) -> SfrOptmaskW<'_, SfrOptmaskSpec> {
        SfrOptmaskW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L305 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L305>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOptmaskSpec;
impl crate::RegisterSpec for SfrOptmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_optmask::R`](R) reader structure"]
impl crate::Readable for SfrOptmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_optmask::W`](W) writer structure"]
impl crate::Writable for SfrOptmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPTMASK to value 0"]
impl crate::Resettable for SfrOptmaskSpec {}
