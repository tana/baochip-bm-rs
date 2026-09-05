#[doc = "Register `CR_XIP_OPCODE` reader"]
pub type R = crate::R<CrXipOpcodeSpec>;
#[doc = "Register `CR_XIP_OPCODE` writer"]
pub type W = crate::W<CrXipOpcodeSpec>;
#[doc = "Field `cr_xip_opcode` reader - cr_xip_opcode read/write control register"]
pub type CrXipOpcodeR = crate::FieldReader<u32>;
#[doc = "Field `cr_xip_opcode` writer - cr_xip_opcode read/write control register"]
pub type CrXipOpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_xip_opcode read/write control register"]
    #[inline(always)]
    pub fn cr_xip_opcode(&self) -> CrXipOpcodeR {
        CrXipOpcodeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_xip_opcode read/write control register"]
    #[inline(always)]
    pub fn cr_xip_opcode(&mut self) -> CrXipOpcodeW<'_, CrXipOpcodeSpec> {
        CrXipOpcodeW::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L194 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L194>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_opcode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_opcode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrXipOpcodeSpec;
impl crate::RegisterSpec for CrXipOpcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_xip_opcode::R`](R) reader structure"]
impl crate::Readable for CrXipOpcodeSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_xip_opcode::W`](W) writer structure"]
impl crate::Writable for CrXipOpcodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_XIP_OPCODE to value 0"]
impl crate::Resettable for CrXipOpcodeSpec {}
