#[doc = "Register `REG_BINCU_SETUP` reader"]
pub type R = crate::R<RegBincuSetupSpec>;
#[doc = "Register `REG_BINCU_SETUP` writer"]
pub type W = crate::W<RegBincuSetupSpec>;
#[doc = "Field `r_bincu_datasize` reader - r_bincu_datasize"]
pub type RBincuDatasizeR = crate::FieldReader;
#[doc = "Field `r_bincu_datasize` writer - r_bincu_datasize"]
pub type RBincuDatasizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - r_bincu_datasize"]
    #[inline(always)]
    pub fn r_bincu_datasize(&self) -> RBincuDatasizeR {
        RBincuDatasizeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - r_bincu_datasize"]
    #[inline(always)]
    pub fn r_bincu_datasize(&mut self) -> RBincuDatasizeW<'_, RegBincuSetupSpec> {
        RBincuDatasizeW::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bincu_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bincu_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBincuSetupSpec;
impl crate::RegisterSpec for RegBincuSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bincu_setup::R`](R) reader structure"]
impl crate::Readable for RegBincuSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bincu_setup::W`](W) writer structure"]
impl crate::Writable for RegBincuSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BINCU_SETUP to value 0"]
impl crate::Resettable for RegBincuSetupSpec {}
