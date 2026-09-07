#[doc = "Register `DONE` reader"]
pub type R = crate::R<DoneSpec>;
#[doc = "Register `DONE` writer"]
pub type W = crate::W<DoneSpec>;
#[doc = "Field `done` reader - Writing a `1` to this field indicates to the corresponding peer that a full packet is done loading. There is no need to clear this register after writing."]
pub type DoneR = crate::BitReader;
#[doc = "Field `done` writer - Writing a `1` to this field indicates to the corresponding peer that a full packet is done loading. There is no need to clear this register after writing."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing a `1` to this field indicates to the corresponding peer that a full packet is done loading. There is no need to clear this register after writing."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a `1` to this field indicates to the corresponding peer that a full packet is done loading. There is no need to clear this register after writing."]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, DoneSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoneSpec;
impl crate::RegisterSpec for DoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`done::R`](R) reader structure"]
impl crate::Readable for DoneSpec {}
#[doc = "`write(|w| ..)` method takes [`done::W`](W) writer structure"]
impl crate::Writable for DoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DONE to value 0"]
impl crate::Resettable for DoneSpec {}
