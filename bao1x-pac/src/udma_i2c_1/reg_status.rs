#[doc = "Register `REG_STATUS` reader"]
pub type R = crate::R<RegStatusSpec>;
#[doc = "Register `REG_STATUS` writer"]
pub type W = crate::W<RegStatusSpec>;
#[doc = "Field `r_busy` reader - r_busy"]
pub type RBusyR = crate::BitReader;
#[doc = "Field `r_busy` writer - r_busy"]
pub type RBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_al` reader - r_al"]
pub type RAlR = crate::BitReader;
#[doc = "Field `r_al` writer - r_al"]
pub type RAlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_busy"]
    #[inline(always)]
    pub fn r_busy(&self) -> RBusyR {
        RBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_al"]
    #[inline(always)]
    pub fn r_al(&self) -> RAlR {
        RAlR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_busy"]
    #[inline(always)]
    pub fn r_busy(&mut self) -> RBusyW<'_, RegStatusSpec> {
        RBusyW::new(self, 0)
    }
    #[doc = "Bit 1 - r_al"]
    #[inline(always)]
    pub fn r_al(&mut self) -> RAlW<'_, RegStatusSpec> {
        RAlW::new(self, 1)
    }
}
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegStatusSpec;
impl crate::RegisterSpec for RegStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_status::R`](R) reader structure"]
impl crate::Readable for RegStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_status::W`](W) writer structure"]
impl crate::Writable for RegStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_STATUS to value 0"]
impl crate::Resettable for RegStatusSpec {}
