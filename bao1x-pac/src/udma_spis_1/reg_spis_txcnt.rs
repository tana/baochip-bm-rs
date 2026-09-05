#[doc = "Register `REG_SPIS_TXCNT` reader"]
pub type R = crate::R<RegSpisTxcntSpec>;
#[doc = "Register `REG_SPIS_TXCNT` writer"]
pub type W = crate::W<RegSpisTxcntSpec>;
#[doc = "Field `cfgtxcnt` reader - cfgtxcnt"]
pub type CfgtxcntR = crate::FieldReader<u16>;
#[doc = "Field `cfgtxcnt` writer - cfgtxcnt"]
pub type CfgtxcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfgtxcnt"]
    #[inline(always)]
    pub fn cfgtxcnt(&self) -> CfgtxcntR {
        CfgtxcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfgtxcnt"]
    #[inline(always)]
    pub fn cfgtxcnt(&mut self) -> CfgtxcntW<'_, RegSpisTxcntSpec> {
        CfgtxcntW::new(self, 0)
    }
}
#[doc = "See `udma_spis_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_spis_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_spis_txcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_spis_txcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpisTxcntSpec;
impl crate::RegisterSpec for RegSpisTxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_spis_txcnt::R`](R) reader structure"]
impl crate::Readable for RegSpisTxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_spis_txcnt::W`](W) writer structure"]
impl crate::Writable for RegSpisTxcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SPIS_TXCNT to value 0"]
impl crate::Resettable for RegSpisTxcntSpec {}
