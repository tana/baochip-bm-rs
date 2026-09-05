#[doc = "Register `REG_BINCU_VAL` reader"]
pub type R = crate::R<RegBincuValSpec>;
#[doc = "Register `REG_BINCU_VAL` writer"]
pub type W = crate::W<RegBincuValSpec>;
#[doc = "Field `bincu_counter_i` reader - bincu_counter_i"]
pub type BincuCounterIR = crate::FieldReader<u16>;
#[doc = "Field `bincu_counter_i` writer - bincu_counter_i"]
pub type BincuCounterIW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - bincu_counter_i"]
    #[inline(always)]
    pub fn bincu_counter_i(&self) -> BincuCounterIR {
        BincuCounterIR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - bincu_counter_i"]
    #[inline(always)]
    pub fn bincu_counter_i(&mut self) -> BincuCounterIW<'_, RegBincuValSpec> {
        BincuCounterIW::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bincu_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bincu_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBincuValSpec;
impl crate::RegisterSpec for RegBincuValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bincu_val::R`](R) reader structure"]
impl crate::Readable for RegBincuValSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bincu_val::W`](W) writer structure"]
impl crate::Writable for RegBincuValSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BINCU_VAL to value 0"]
impl crate::Resettable for RegBincuValSpec {}
