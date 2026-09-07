#[doc = "Register `EV_POLARITY` reader"]
pub type R = crate::R<EvPolaritySpec>;
#[doc = "Register `EV_POLARITY` writer"]
pub type W = crate::W<EvPolaritySpec>;
#[doc = "Field `rising` reader - None"]
pub type RisingR = crate::FieldReader<u16>;
#[doc = "Field `rising` writer - None"]
pub type RisingW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - None"]
    #[inline(always)]
    pub fn rising(&self) -> RisingR {
        RisingR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - None"]
    #[inline(always)]
    pub fn rising(&mut self) -> RisingW<'_, EvPolaritySpec> {
        RisingW::new(self, 0)
    }
}
#[doc = "If a bit is set to 1, then the polarity is rising edge triggered; 0 is falling edge triggered. Bit is ignored if `edge_triggered` is 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_polarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_polarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvPolaritySpec;
impl crate::RegisterSpec for EvPolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_polarity::R`](R) reader structure"]
impl crate::Readable for EvPolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`ev_polarity::W`](W) writer structure"]
impl crate::Writable for EvPolaritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_POLARITY to value 0"]
impl crate::Resettable for EvPolaritySpec {}
