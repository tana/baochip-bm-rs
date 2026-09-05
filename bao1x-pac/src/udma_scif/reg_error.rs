#[doc = "Register `REG_ERROR` reader"]
pub type R = crate::R<RegErrorSpec>;
#[doc = "Register `REG_ERROR` writer"]
pub type W = crate::W<RegErrorSpec>;
#[doc = "Field `r_err_overflow` reader - r_err_overflow"]
pub type RErrOverflowR = crate::BitReader;
#[doc = "Field `r_err_overflow` writer - r_err_overflow"]
pub type RErrOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_err_parity` reader - r_err_parity"]
pub type RErrParityR = crate::BitReader;
#[doc = "Field `r_err_parity` writer - r_err_parity"]
pub type RErrParityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_err_overflow"]
    #[inline(always)]
    pub fn r_err_overflow(&self) -> RErrOverflowR {
        RErrOverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_err_parity"]
    #[inline(always)]
    pub fn r_err_parity(&self) -> RErrParityR {
        RErrParityR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_err_overflow"]
    #[inline(always)]
    pub fn r_err_overflow(&mut self) -> RErrOverflowW<'_, RegErrorSpec> {
        RErrOverflowW::new(self, 0)
    }
    #[doc = "Bit 1 - r_err_parity"]
    #[inline(always)]
    pub fn r_err_parity(&mut self) -> RErrParityW<'_, RegErrorSpec> {
        RErrParityW::new(self, 1)
    }
}
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_error::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_error::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegErrorSpec;
impl crate::RegisterSpec for RegErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_error::R`](R) reader structure"]
impl crate::Readable for RegErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_error::W`](W) writer structure"]
impl crate::Writable for RegErrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ERROR to value 0"]
impl crate::Resettable for RegErrorSpec {}
