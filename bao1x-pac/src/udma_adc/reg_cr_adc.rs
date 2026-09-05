#[doc = "Register `REG_CR_ADC` reader"]
pub type R = crate::R<RegCrAdcSpec>;
#[doc = "Register `REG_CR_ADC` writer"]
pub type W = crate::W<RegCrAdcSpec>;
#[doc = "Field `cr_adc` reader - cr_adc"]
pub type CrAdcR = crate::FieldReader<u32>;
#[doc = "Field `cr_adc` writer - cr_adc"]
pub type CrAdcW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - cr_adc"]
    #[inline(always)]
    pub fn cr_adc(&self) -> CrAdcR {
        CrAdcR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - cr_adc"]
    #[inline(always)]
    pub fn cr_adc(&mut self) -> CrAdcW<'_, RegCrAdcSpec> {
        CrAdcW::new(self, 0)
    }
}
#[doc = "See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cr_adc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cr_adc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCrAdcSpec;
impl crate::RegisterSpec for RegCrAdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cr_adc::R`](R) reader structure"]
impl crate::Readable for RegCrAdcSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cr_adc::W`](W) writer structure"]
impl crate::Writable for RegCrAdcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CR_ADC to value 0"]
impl crate::Resettable for RegCrAdcSpec {}
