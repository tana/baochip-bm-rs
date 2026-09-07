#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `pause` reader - Write a `1` to this field to request a pause to counting, 0 for free-run. Count pauses on the next tick quanta."]
pub type PauseR = crate::BitReader;
#[doc = "Field `pause` writer - Write a `1` to this field to request a pause to counting, 0 for free-run. Count pauses on the next tick quanta."]
pub type PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `load` reader - If paused, write a `1` to this bit to load a resume value to the timer. If not paused, this bit is ignored."]
pub type LoadR = crate::BitReader;
#[doc = "Field `load` writer - If paused, write a `1` to this bit to load a resume value to the timer. If not paused, this bit is ignored."]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a `1` to this field to request a pause to counting, 0 for free-run. Count pauses on the next tick quanta."]
    #[inline(always)]
    pub fn pause(&self) -> PauseR {
        PauseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If paused, write a `1` to this bit to load a resume value to the timer. If not paused, this bit is ignored."]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a `1` to this field to request a pause to counting, 0 for free-run. Count pauses on the next tick quanta."]
    #[inline(always)]
    pub fn pause(&mut self) -> PauseW<'_, ControlSpec> {
        PauseW::new(self, 0)
    }
    #[doc = "Bit 1 - If paused, write a `1` to this bit to load a resume value to the timer. If not paused, this bit is ignored."]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, ControlSpec> {
        LoadW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {}
