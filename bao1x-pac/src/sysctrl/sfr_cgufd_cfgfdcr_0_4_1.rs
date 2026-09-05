#[doc = "Register `SFR_CGUFD_CFGFDCR_0_4_1` reader"]
pub type R = crate::R<SfrCgufdCfgfdcr0_4_1Spec>;
#[doc = "Register `SFR_CGUFD_CFGFDCR_0_4_1` writer"]
pub type W = crate::W<SfrCgufdCfgfdcr0_4_1Spec>;
#[doc = "Field `cfgfdcr_0_4_1` reader - cfgfdcr read/write control register"]
pub type Cfgfdcr0_4_1R = crate::FieldReader<u32>;
#[doc = "Field `cfgfdcr_0_4_1` writer - cfgfdcr read/write control register"]
pub type Cfgfdcr0_4_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cfgfdcr read/write control register"]
    #[inline(always)]
    pub fn cfgfdcr_0_4_1(&self) -> Cfgfdcr0_4_1R {
        Cfgfdcr0_4_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cfgfdcr read/write control register"]
    #[inline(always)]
    pub fn cfgfdcr_0_4_1(&mut self) -> Cfgfdcr0_4_1W<'_, SfrCgufdCfgfdcr0_4_1Spec> {
        Cfgfdcr0_4_1W::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufd_cfgfdcr_0_4_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufd_cfgfdcr_0_4_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufdCfgfdcr0_4_1Spec;
impl crate::RegisterSpec for SfrCgufdCfgfdcr0_4_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufd_cfgfdcr_0_4_1::R`](R) reader structure"]
impl crate::Readable for SfrCgufdCfgfdcr0_4_1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufd_cfgfdcr_0_4_1::W`](W) writer structure"]
impl crate::Writable for SfrCgufdCfgfdcr0_4_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFD_CFGFDCR_0_4_1 to value 0"]
impl crate::Resettable for SfrCgufdCfgfdcr0_4_1Spec {}
