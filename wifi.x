   
SECTIONS {
  .rwtext_wifi :
  {
   . = ALIGN(4);
        *( .wifi0iram  .wifi0iram.*)
        *( .wifirxiram  .wifirxiram.*)
        *( .iram1  .iram1.*)
  } > RWTEXT

  .data_wifi :
  {
   . = ALIGN(4);
        *( .dram1 .dram1.*)
  } > RWDATA
}

