#[doc = "Register `STATE` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Register `STATE` writer"]
pub type W = crate::W<StateSpec>;
#[doc = "Field `resume` reader - Used to transfer the resume state information from the loader to Xous. If set, indicates we are on the resume half of a suspend/resume."]
pub type ResumeR = crate::BitReader;
#[doc = "Field `resume` writer - Used to transfer the resume state information from the loader to Xous. If set, indicates we are on the resume half of a suspend/resume."]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `was_forced` reader - Used by the bootloader to indicate to the kernel if the current resume was from a forced suspend (e.g. a timeout happened and a server may be unclean."]
pub type WasForcedR = crate::BitReader;
#[doc = "Field `was_forced` writer - Used by the bootloader to indicate to the kernel if the current resume was from a forced suspend (e.g. a timeout happened and a server may be unclean."]
pub type WasForcedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Used to transfer the resume state information from the loader to Xous. If set, indicates we are on the resume half of a suspend/resume."]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Used by the bootloader to indicate to the kernel if the current resume was from a forced suspend (e.g. a timeout happened and a server may be unclean."]
    #[inline(always)]
    pub fn was_forced(&self) -> WasForcedR {
        WasForcedR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to transfer the resume state information from the loader to Xous. If set, indicates we are on the resume half of a suspend/resume."]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<'_, StateSpec> {
        ResumeW::new(self, 0)
    }
    #[doc = "Bit 1 - Used by the bootloader to indicate to the kernel if the current resume was from a forced suspend (e.g. a timeout happened and a server may be unclean."]
    #[inline(always)]
    pub fn was_forced(&mut self) -> WasForcedW<'_, StateSpec> {
        WasForcedW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`write(|w| ..)` method takes [`state::W`](W) writer structure"]
impl crate::Writable for StateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for StateSpec {}
