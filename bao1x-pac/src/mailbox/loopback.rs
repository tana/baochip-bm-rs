#[doc = "Register `LOOPBACK` reader"]
pub type R = crate::R<LoopbackSpec>;
#[doc = "Register `LOOPBACK` writer"]
pub type W = crate::W<LoopbackSpec>;
#[doc = "Field `loopback` reader - Writing a `1` to this field indicates that the mailbox should loopback to the local client. `0` connects it to the external core."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `loopback` writer - Writing a `1` to this field indicates that the mailbox should loopback to the local client. `0` connects it to the external core."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing a `1` to this field indicates that the mailbox should loopback to the local client. `0` connects it to the external core."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a `1` to this field indicates that the mailbox should loopback to the local client. `0` connects it to the external core."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LoopbackW<'_, LoopbackSpec> {
        LoopbackW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`loopback::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loopback::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoopbackSpec;
impl crate::RegisterSpec for LoopbackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loopback::R`](R) reader structure"]
impl crate::Readable for LoopbackSpec {}
#[doc = "`write(|w| ..)` method takes [`loopback::W`](W) writer structure"]
impl crate::Writable for LoopbackSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOOPBACK to value 0"]
impl crate::Resettable for LoopbackSpec {}
