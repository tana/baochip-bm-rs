#[doc = "Register `REG_BINCU_CNT` reader"]
pub type R = crate::R<RegBincuCntSpec>;
#[doc = "Register `REG_BINCU_CNT` writer"]
pub type W = crate::W<RegBincuCntSpec>;
#[doc = "Field `r_bincu_counter` reader - r_bincu_counter"]
pub type RBincuCounterR = crate::FieldReader<u16>;
#[doc = "Field `r_bincu_counter` writer - r_bincu_counter"]
pub type RBincuCounterW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `r_bincu_en_counter` reader - r_bincu_en_counter"]
pub type RBincuEnCounterR = crate::BitReader;
#[doc = "Field `r_bincu_en_counter` writer - r_bincu_en_counter"]
pub type RBincuEnCounterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - r_bincu_counter"]
    #[inline(always)]
    pub fn r_bincu_counter(&self) -> RBincuCounterR {
        RBincuCounterR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - r_bincu_en_counter"]
    #[inline(always)]
    pub fn r_bincu_en_counter(&self) -> RBincuEnCounterR {
        RBincuEnCounterR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - r_bincu_counter"]
    #[inline(always)]
    pub fn r_bincu_counter(&mut self) -> RBincuCounterW<'_, RegBincuCntSpec> {
        RBincuCounterW::new(self, 0)
    }
    #[doc = "Bit 31 - r_bincu_en_counter"]
    #[inline(always)]
    pub fn r_bincu_en_counter(&mut self) -> RBincuEnCounterW<'_, RegBincuCntSpec> {
        RBincuEnCounterW::new(self, 31)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bincu_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bincu_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBincuCntSpec;
impl crate::RegisterSpec for RegBincuCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bincu_cnt::R`](R) reader structure"]
impl crate::Readable for RegBincuCntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bincu_cnt::W`](W) writer structure"]
impl crate::Writable for RegBincuCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BINCU_CNT to value 0"]
impl crate::Resettable for RegBincuCntSpec {}
