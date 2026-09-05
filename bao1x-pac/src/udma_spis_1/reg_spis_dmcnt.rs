#[doc = "Register `REG_SPIS_DMCNT` reader"]
pub type R = crate::R<RegSpisDmcntSpec>;
#[doc = "Register `REG_SPIS_DMCNT` writer"]
pub type W = crate::W<RegSpisDmcntSpec>;
#[doc = "Field `cfgdmcnt` reader - cfgdmcnt"]
pub type CfgdmcntR = crate::FieldReader<u16>;
#[doc = "Field `cfgdmcnt` writer - cfgdmcnt"]
pub type CfgdmcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cfgdmcnt"]
    #[inline(always)]
    pub fn cfgdmcnt(&self) -> CfgdmcntR {
        CfgdmcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cfgdmcnt"]
    #[inline(always)]
    pub fn cfgdmcnt(&mut self) -> CfgdmcntW<'_, RegSpisDmcntSpec> {
        CfgdmcntW::new(self, 0)
    }
}
#[doc = "See `udma_spis_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_spis_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_spis_dmcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_spis_dmcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpisDmcntSpec;
impl crate::RegisterSpec for RegSpisDmcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_spis_dmcnt::R`](R) reader structure"]
impl crate::Readable for RegSpisDmcntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_spis_dmcnt::W`](W) writer structure"]
impl crate::Writable for RegSpisDmcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SPIS_DMCNT to value 0"]
impl crate::Resettable for RegSpisDmcntSpec {}
