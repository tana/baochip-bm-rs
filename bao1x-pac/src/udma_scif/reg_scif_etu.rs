#[doc = "Register `REG_SCIF_ETU` reader"]
pub type R = crate::R<RegScifEtuSpec>;
#[doc = "Register `REG_SCIF_ETU` writer"]
pub type W = crate::W<RegScifEtuSpec>;
#[doc = "Field `r_scif_etu` reader - r_scif_etu"]
pub type RScifEtuR = crate::FieldReader<u16>;
#[doc = "Field `r_scif_etu` writer - r_scif_etu"]
pub type RScifEtuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - r_scif_etu"]
    #[inline(always)]
    pub fn r_scif_etu(&self) -> RScifEtuR {
        RScifEtuR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_scif_etu"]
    #[inline(always)]
    pub fn r_scif_etu(&mut self) -> RScifEtuW<'_, RegScifEtuSpec> {
        RScifEtuW::new(self, 0)
    }
}
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_scif_etu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_scif_etu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegScifEtuSpec;
impl crate::RegisterSpec for RegScifEtuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_scif_etu::R`](R) reader structure"]
impl crate::Readable for RegScifEtuSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_scif_etu::W`](W) writer structure"]
impl crate::Writable for RegScifEtuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SCIF_ETU to value 0"]
impl crate::Resettable for RegScifEtuSpec {}
