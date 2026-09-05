#[doc = "Register `REG_SPIS_RXCNT` reader"]
pub type R = crate::R<RegSpisRxcntSpec>;
#[doc = "Register `REG_SPIS_RXCNT` writer"]
pub type W = crate::W<RegSpisRxcntSpec>;
#[doc = "Field `cfgrxcnt` reader - cfgrxcnt"]
pub type CfgrxcntR = crate::FieldReader<u16>;
#[doc = "Field `cfgrxcnt` writer - cfgrxcnt"]
pub type CfgrxcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfgrxcnt"]
    #[inline(always)]
    pub fn cfgrxcnt(&self) -> CfgrxcntR {
        CfgrxcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfgrxcnt"]
    #[inline(always)]
    pub fn cfgrxcnt(&mut self) -> CfgrxcntW<'_, RegSpisRxcntSpec> {
        CfgrxcntW::new(self, 0)
    }
}
#[doc = "See `udma_spis_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_spis_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_spis_rxcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_spis_rxcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpisRxcntSpec;
impl crate::RegisterSpec for RegSpisRxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_spis_rxcnt::R`](R) reader structure"]
impl crate::Readable for RegSpisRxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_spis_rxcnt::W`](W) writer structure"]
impl crate::Writable for RegSpisRxcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SPIS_RXCNT to value 0"]
impl crate::Resettable for RegSpisRxcntSpec {}
