#[doc = "Register `REG_DATA_TIMEOUT` reader"]
pub type R = crate::R<RegDataTimeoutSpec>;
#[doc = "Register `REG_DATA_TIMEOUT` writer"]
pub type W = crate::W<RegDataTimeoutSpec>;
#[doc = "Field `r_data_timeout` reader - r_data_timeout"]
pub type RDataTimeoutR = crate::FieldReader<u32>;
#[doc = "Field `r_data_timeout` writer - r_data_timeout"]
pub type RDataTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - r_data_timeout"]
    #[inline(always)]
    pub fn r_data_timeout(&self) -> RDataTimeoutR {
        RDataTimeoutR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - r_data_timeout"]
    #[inline(always)]
    pub fn r_data_timeout(&mut self) -> RDataTimeoutW<'_, RegDataTimeoutSpec> {
        RDataTimeoutW::new(self, 0)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_data_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_data_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDataTimeoutSpec;
impl crate::RegisterSpec for RegDataTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_data_timeout::R`](R) reader structure"]
impl crate::Readable for RegDataTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_data_timeout::W`](W) writer structure"]
impl crate::Writable for RegDataTimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DATA_TIMEOUT to value 0"]
impl crate::Resettable for RegDataTimeoutSpec {}
