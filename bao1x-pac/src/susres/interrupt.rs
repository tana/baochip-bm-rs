#[doc = "Register `INTERRUPT` reader"]
pub type R = crate::R<InterruptSpec>;
#[doc = "Register `INTERRUPT` writer"]
pub type W = crate::W<InterruptSpec>;
#[doc = "Field `interrupt` reader - Writing this causes an interrupt to fire. Used by Xous to initiate suspend/resume from an interrupt context."]
pub type InterruptR = crate::BitReader;
#[doc = "Field `interrupt` writer - Writing this causes an interrupt to fire. Used by Xous to initiate suspend/resume from an interrupt context."]
pub type InterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing this causes an interrupt to fire. Used by Xous to initiate suspend/resume from an interrupt context."]
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing this causes an interrupt to fire. Used by Xous to initiate suspend/resume from an interrupt context."]
    #[inline(always)]
    pub fn interrupt(&mut self) -> InterruptW<'_, InterruptSpec> {
        InterruptW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptSpec;
impl crate::RegisterSpec for InterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for InterruptSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt::W`](W) writer structure"]
impl crate::Writable for InterruptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for InterruptSpec {}
