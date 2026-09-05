#[doc = "Register `REG_STATUS` reader"]
pub type R = crate::R<RegStatusSpec>;
#[doc = "Register `REG_STATUS` writer"]
pub type W = crate::W<RegStatusSpec>;
#[doc = "Field `r_eot` reader - r_eot"]
pub type REotR = crate::BitReader;
#[doc = "Field `r_eot` writer - r_eot"]
pub type REotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_err` reader - r_err"]
pub type RErrR = crate::BitReader;
#[doc = "Field `r_err` writer - r_err"]
pub type RErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_eot"]
    #[inline(always)]
    pub fn r_eot(&self) -> REotR {
        REotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_err"]
    #[inline(always)]
    pub fn r_err(&self) -> RErrR {
        RErrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_eot"]
    #[inline(always)]
    pub fn r_eot(&mut self) -> REotW<'_, RegStatusSpec> {
        REotW::new(self, 0)
    }
    #[doc = "Bit 1 - r_err"]
    #[inline(always)]
    pub fn r_err(&mut self) -> RErrW<'_, RegStatusSpec> {
        RErrW::new(self, 1)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
