#[doc = "Register `REG_RST` reader"]
pub type R = crate::R<RegRstSpec>;
#[doc = "Register `REG_RST` writer"]
pub type W = crate::W<RegRstSpec>;
#[doc = "Field `r_rst` reader - r_rst"]
pub type RRstR = crate::FieldReader;
#[doc = "Field `r_rst` writer - r_rst"]
pub type RRstW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - r_rst"]
    #[inline(always)]
    pub fn r_rst(&self) -> RRstR {
        RRstR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - r_rst"]
    #[inline(always)]
    pub fn r_rst(&mut self) -> RRstW<'_, RegRstSpec> {
        RRstW::new(self, 0)
    }
}
#[doc = "See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRstSpec;
impl crate::RegisterSpec for RegRstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rst::R`](R) reader structure"]
impl crate::Readable for RegRstSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rst::W`](W) writer structure"]
impl crate::Writable for RegRstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RST to value 0"]
impl crate::Resettable for RegRstSpec {}
