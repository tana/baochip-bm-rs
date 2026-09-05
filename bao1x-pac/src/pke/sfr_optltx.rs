#[doc = "Register `SFR_OPTLTX` reader"]
pub type R = crate::R<SfrOptltxSpec>;
#[doc = "Register `SFR_OPTLTX` writer"]
pub type W = crate::W<SfrOptltxSpec>;
#[doc = "Field `sfr_optltx` reader - sfr_optltx read/write control register"]
pub type SfrOptltxR = crate::FieldReader;
#[doc = "Field `sfr_optltx` writer - sfr_optltx read/write control register"]
pub type SfrOptltxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sfr_optltx read/write control register"]
    #[inline(always)]
    pub fn sfr_optltx(&self) -> SfrOptltxR {
        SfrOptltxR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sfr_optltx read/write control register"]
    #[inline(always)]
    pub fn sfr_optltx(&mut self) -> SfrOptltxW<'_, SfrOptltxSpec> {
        SfrOptltxW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L303 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L303>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optltx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optltx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOptltxSpec;
impl crate::RegisterSpec for SfrOptltxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_optltx::R`](R) reader structure"]
impl crate::Readable for SfrOptltxSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_optltx::W`](W) writer structure"]
impl crate::Writable for SfrOptltxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPTLTX to value 0"]
impl crate::Resettable for SfrOptltxSpec {}
