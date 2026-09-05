#[doc = "Register `SFR_ARCLR` reader"]
pub type R = crate::R<SfrArclrSpec>;
#[doc = "Register `SFR_ARCLR` writer"]
pub type W = crate::W<SfrArclrSpec>;
#[doc = "Field `ar_clrram` reader - ar_clrram performs action on write of value: 0xa5"]
pub type ArClrramR = crate::FieldReader<u32>;
#[doc = "Field `ar_clrram` writer - ar_clrram performs action on write of value: 0xa5"]
pub type ArClrramW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ar_clrram performs action on write of value: 0xa5"]
    #[inline(always)]
    pub fn ar_clrram(&self) -> ArClrramR {
        ArClrramR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ar_clrram performs action on write of value: 0xa5"]
    #[inline(always)]
    pub fn ar_clrram(&mut self) -> ArClrramW<'_, SfrArclrSpec> {
        ArClrramW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L84 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L84>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_arclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_arclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrArclrSpec;
impl crate::RegisterSpec for SfrArclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_arclr::R`](R) reader structure"]
impl crate::Readable for SfrArclrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_arclr::W`](W) writer structure"]
impl crate::Writable for SfrArclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ARCLR to value 0"]
impl crate::Resettable for SfrArclrSpec {}
