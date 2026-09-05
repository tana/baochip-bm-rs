#[doc = "Register `SFR_FFCLR` reader"]
pub type R = crate::R<SfrFfclrSpec>;
#[doc = "Register `SFR_FFCLR` writer"]
pub type W = crate::W<SfrFfclrSpec>;
#[doc = "Field `ar_ffclr` reader - ar_ffclr performs action on write of value: (32'hff00+i)"]
pub type ArFfclrR = crate::FieldReader<u32>;
#[doc = "Field `ar_ffclr` writer - ar_ffclr performs action on write of value: (32'hff00+i)"]
pub type ArFfclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ar_ffclr performs action on write of value: (32'hff00+i)"]
    #[inline(always)]
    pub fn ar_ffclr(&self) -> ArFfclrR {
        ArFfclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ar_ffclr performs action on write of value: (32'hff00+i)"]
    #[inline(always)]
    pub fn ar_ffclr(&mut self) -> ArFfclrW<'_, SfrFfclrSpec> {
        ArFfclrW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L96 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L96>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFfclrSpec;
impl crate::RegisterSpec for SfrFfclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ffclr::R`](R) reader structure"]
impl crate::Readable for SfrFfclrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ffclr::W`](W) writer structure"]
impl crate::Writable for SfrFfclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FFCLR to value 0"]
impl crate::Resettable for SfrFfclrSpec {}
