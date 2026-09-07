#[doc = "Register `EV_SOFT` reader"]
pub type R = crate::R<EvSoftSpec>;
#[doc = "Register `EV_SOFT` writer"]
pub type W = crate::W<EvSoftSpec>;
#[doc = "Field `trigger` reader - None"]
pub type TriggerR = crate::FieldReader<u16>;
#[doc = "Field `trigger` writer - None"]
pub type TriggerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - None"]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - None"]
    #[inline(always)]
    pub fn trigger(&mut self) -> TriggerW<'_, EvSoftSpec> {
        TriggerW::new(self, 0)
    }
}
#[doc = "Software interrupt trigger register. ) Bits set to `1` will trigger an interrupt. Interrupts trigger on write, but the value will persist in the register, allowing software to determine if a software interrupt was triggered by reading back the register. Software is responsible for clearing the register to 0. Repeated `1` writes without clearing will still trigger an interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_soft::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_soft::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvSoftSpec;
impl crate::RegisterSpec for EvSoftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_soft::R`](R) reader structure"]
impl crate::Readable for EvSoftSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_soft::W`](W) writer structure"]
impl crate::Writable for EvSoftSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_SOFT to value 0"]
impl crate::Resettable for EvSoftSpec {}
