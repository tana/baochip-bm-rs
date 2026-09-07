#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `enable` reader - When set to `1`, mappings are enabled. When `0`, the `CoreUser` value is fixed to 0b0001_0000, and the `vex_mm` bit is set to '`1`."]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - When set to `1`, mappings are enabled. When `0`, the `CoreUser` value is fixed to 0b0001_0000, and the `vex_mm` bit is set to '`1`."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `invert_priv` reader - When set to `1` inverts the sense of the privilege bit"]
pub type InvertPrivR = crate::BitReader;
#[doc = "Field `invert_priv` writer - When set to `1` inverts the sense of the privilege bit"]
pub type InvertPrivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to `1`, mappings are enabled. When `0`, the `CoreUser` value is fixed to 0b0001_0000, and the `vex_mm` bit is set to '`1`."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to `1` inverts the sense of the privilege bit"]
    #[inline(always)]
    pub fn invert_priv(&self) -> InvertPrivR {
        InvertPrivR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to `1`, mappings are enabled. When `0`, the `CoreUser` value is fixed to 0b0001_0000, and the `vex_mm` bit is set to '`1`."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to `1` inverts the sense of the privilege bit"]
    #[inline(always)]
    pub fn invert_priv(&mut self) -> InvertPrivW<'_, ControlSpec> {
        InvertPrivW::new(self, 1)
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
