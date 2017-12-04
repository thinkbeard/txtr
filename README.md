# txtr

txtr converts images to text.

$ txtr -w 40 -f 0.3 assets/castle.jpg
```
...,,...................................
...,........../.........................
..............%%{%......................
............/%%%/%{,....................
.........;{%%{//;/{%{%%.................
...........#####{%%#%...................
.......;%{{%%%%#%%%%/%%%................
..........%%%#%#####%;{.................
.....;%{{{{/#%%/{%%{;%#%%.............;/
.......%%%/##%###%%;%%#{%.......,.;..://
....,{%%%%%%/{%%{//%%##%%%%../..,{;%,%{{
.....{{{###############%%%{%{%%/./{..%%{
///%{%%#%%%%%%%%%%%%%%%%{{%%%%#/#%{{%%{%
/{{{###%###%##%##%#%####%##%%%###{%%#{{#
%{%{{%%{#%%%%%%%;;{%%;;;;/%{//##{%##%{%{
##%%%#{{{%%%%%/{/%%;;{{{{/%//;::%###;{%%
```

$ txtr -w 40 -f 1.0 assets/castle.jpg

```
{;::....................................
,..;,...................................
........................................
.,......................................
........................................
.,............:.........................
..............%..,......................
..............%%./......................
..............%%%%......................
.............{{%%%......................
.............{%{%{%.....................
............/%%%%%{.....................
...........%%%%//{{{....................
.........%%{/%%{;#{{%...................
.........{//%%%{/{%{%{%.................
..........///;;;%{/{%%;.................
...........{%%###%%{;...................
...........%#####%%#%...................
...........%##%{%{%%%...................
..........%;{%%%%%#%%...................
.........%{{#{%%%{/%%%..................
.......;%{{%%%{#%%%%/%%{................
.......,;%%{;%%%%{{%##%%{...............
..........{%{%/%%###%/..................
..........%{{%%#####%;%.................
..........%%%#%#%#%#%%{.................
.........%{%%%{%%%%#%%{................/
.......%%%{%%{{{/%{%%%:{..............,;
.....%{{/%%/%{%%%%{%%%%%%{............/{
...../{{%%%;%%%%{%%/{%%{{%%...........;{
......./{%{:;%#%%#%;%%{{%............%/{
.......%%%/##{###{{;%%#{%.......,.;,.,//
.......%%/###/%#{%;%%####.......:%{{///%
......,%{;####;%#;{{%%%%%%;..,,./%{%;:{%
.....%%%/%%%%%%%%%%/%{%#%{%..;,.;%%;{{{{
...{%{{/{{{{%%%%%%%%{%###%;,,:...{;%/.;%
...%%%{//%{%%%%%{%#%####%%;%{.,..{%;:{{{
....,%%{{{{{%#%########%{%%%{{#,.;%{{{%%
.....%%%###############%%%%.%{%%./{,,%{{
/////%#%#######/#{###%%%%/%{%%%%%%{;//%%
/////##%%#%%###%{%##%%{//{{/%/%%%%%%%%%{
///%%%##%%%%%%{%%%%%%%%%%{%%%##/#%%{%%{%
////{{%%%%%%%%%#{{{{/%###%%%%%###%{//%/#
/////{{{%////{%######%##%#%{%%##%{/%%{%#
////%%#####%##%##%%##%#%###%#%###{%{%{{#
%{{{%%#%######%#%####%%%%%%%#%%##%%%{{{#
%%{{#%###%%####%#####/{/#//%{{##{#{%/;//
{%{{%%#%%####%%%%%%%%;;//{{{//##/{#%#{%#
%%%{%%%%#%%%%%%%%{%%#/;;;;////##;%{%%%%#
{%{%{%%#%#%#%##/{;%;/%%/{%/%%{//%%%%##%%
#%%{/{%{{%%%%#%/#/{{{;%{/{//%{,{%%{{{#%/
##/{%%%{%%%%%%%{%%/;;%%%%{{;%;::####%%%%
#%#%//%%{%%%%%{;:{;;/;;%;;/{::%/%/%##%%%
```

$ txtr -w 100 -f 0.3 assets/castle.jpg

```
%.%::./,:...........................................................................................
..{......:.,,,......................................................................................
....................................................................................................
......,..:..........................................................................................
...................................{................................................................
..................................%%%%%,....:.......................................................
..................................{{%%{%%%%%%.......................................................
................................/////;%%%{{{%%......................................................
...............................{{%%%%%%;%{/{{{{%....................................................
...........................;%//%%%%{{{//{{%/%{{{{%/.................................................
.......................%%{{{{%%%{{{///{%%%%%%%{{%%{{%%..............................................
.........................{%%{{{{//////;;;;;;;/{{{%%%%//{/...........................................
...........................;%%%%#%##########%#%%%;;;;...............................................
...........................%%%############%%%#%##%%%%...............................................
..........................{%{{/{%{%%%%%%#%%#%/%%%%%%%...............................................
....................../%%%%%%%%%%%%%%%%{%%%{/%//%%%%%%%%............................................
................../%{%%%%%{{%{;/{{;%%##%%%%%%{%%%###%#%%%%%%%.......................................
.......................%%{%;;%%##%%/;/%%%##%%###%###%%{%%%%{/.......................................
.........................{%%{//{%%%%%%######%#####%%%;{{/...........................................
......................../%%%%%%%%%%%%%#######%%##%%/%%#%{{.........................................,
...................;%%{{{%%%%%%%#%#%%%%%%{{{{{%%%%%%%#%%%{%........................................%
.............{%%///{{{{{{%%#;;%##{{{%%%%%%%%%{//{%%%%%;;%%%%%%%%..............................,/{/{%
...............{%%%%%%%%%%//;/;%{%#%%%#%%%%%%{%{;/%%/%%%/%%%%%%/{{..............................;{{;
..................%%%%%%%{;{#;#//%{%%###%##{%%/;/%%/%%%%/{%%#%....................;.{.,;.....:{{/{{{
..................%#%%%%//#######%;/%%##%/%%%%{;;%%%/%#####%%//.........,,..,..../;{%/{{%{/..;///%{:
.............../%%//{/;{#########%%%{%%/{%%/;/%{/{/%%#%%%%%%%%%%%/.....,,:.:.:..{{{%%%%%%{{//{{{.{{%
..........{%%{{%{{%/{{{%{{{%{{{%{{{/{/{{//%%%%%%%%%%%%######%#%%#{%.....,,/::,..,%,/%.{%{../;;%%%{{{
......{%%%%%%%%%{{{//{{%%%%%%%%%%%%%%%%%%{/{%###%%%#######%%%%%%%{,%{%%/:.:,%;.../{%;:%;;{{;%%//{{%#
............{{{{{{{{{{{%%##%##%%%#%#####################%%%%%{%%{/%;%%%{%%{,/%,....{%{::{%%;{%%%%%%{
;;;/;//:,.:/%%%%%%%%####################{%{;;%%%######%%%%%%%{%%{%//;#{#%%%{#%%%///{%/{{/{//:{;;,,{{
////////////%%##############%%%%%%%%%#{%%{{%%{%%%%##%#%%%{{/{{/{{{{{/{/%#{%{%%%%%##%%%{{%%%{%{/%%{%%
///////%{%%%%%%%#%%%%%%%%##%%%%{{{{{{/{%%%%%%%%%%%%%%%########%%%#%%%%{###%%#%%%#/{%{%%%{/{%{{#%{%%%
///////////{%%%{%%%%#%%{{{{{{{{{{{{{##%//////%%{{{%%#%######;{{#/%/%%###%{{%####{{{%{%{%%%%%%%%%{%##
{//////////%%%%%##%%#%#%##########%%%%%%%##%%%######%%#######%#####%####{%{%###%#%%{{%%%{{%%#%%{%%#{
{{{{{{{{{/#######%%####%%%%%%%###%%%%%#%%############%%%###%%%#%%%{%##%%%%{%%#####%#%%{%#%{{%%%%%%#;
%%%%{{{{{{%%%%%###########################/%%%%%%%%%%;//////#{#////;#/////{%####{%%%%{##%#%##%{%%%%%
%%%%%%{{{{{%%#%%%%{%%%%%%%%%#%%%%%%%#%%%{%/%/%%%%%%{%%;;;;;;;;;;/#%#///////%#%##{;{%#/{{%{%%#%#%###{
%%%%%%{{{%{{%%%%{#%%%%%%%%%%%%%%%%#%%%%#{/%;//;{%/{;%/%%%%{%%;;;;/;;/;/;;;;;:%/%/##%%%{/%%{//%%#%###
%###%%%%%%%//%%%%%/{;{{%%%##%%%%%%%%{%{{;;/{{/%;%{{;:{{{{#/%{/{/:/%//,,%:,{/;,;;:%###%#%#%#%##%####%
####{%%%{%%/{/{%%%%%{{{%%%%%%%%%%%{%%{{{/{/::{{{%{/{/{/{:{%;{%,{/{:{{/{//,/;#%;/%%%/##%%#%%%%##%#%#%
```

$ txtr -c ";:,. " -w 100 -f 0.3 assets/castle.jpg

```
; :.. ,..   .                                                                                       
  :      . ...                                                                                      
   .                                                                                                
      .. .                                                                                          
                                   :                                                                
                                  ;;;;:.    ,                                                       
                                  ::;;:;;;;;;                                                       
                                ::,::,;;;::::;                                                      
                              .::;;;;;;,:::::::;                                                    
                           ,:::;;;::::::::;:;::::;:                                                 
                       ;;::::;;;:::,,::;;;;:;:::;;::;;                                              
                         :;:::::::,,,,,,,,,,,:::::;;;:::,                                           
                           ,:;;;;;;;;;;;;;;;;;;;;,,,,                                               
                           ;;;;;;;;;;;;;;;;;;;;;;;;;;                                               
                          :;::::;:;;;;;;;;;;;,;;;:;;;                                               
                      :;;;:::::;;;:;;;;:;;;:,;::;;;;;;;;                                            
                  ::::;;;:::;:,,::,;;;;;;;;;;:;;;;;;;;;;;::;;                                       
                       ;;:;,,;;;;;:,,,;:;;;;;;;;;;;;;;:;;;;::                                       
                         :;;::,:;;;;;;;;;;;;;;;;;;;;;,::,                                           
                        :;;;;;;;;;;;;;;;;;;;;;:;;;;:;;;;::                                         .
                   ,;;::::;:;;;;;;;;;;;;;:::::;;;;;;;;;;;:;                                        ;
             :;;:::::::::;;;,,;;;:::;;;;;;;;;:::::;;;;,,;;;;;;;;                              .::::;
               :;;;;::::;;,,,:,;:;;;;;;;;;;;;:;:,:;;::;:,;;;;:;:::                              ,::,
                  :;;;;;;:,:;,;:,;:;;;;;;;;:;::,:;:,;:;;::;;;;                    , : .,     .::::::
                  ;;;;;;:,;;;;;;;;,,;;;;;,;;:::,,;;;,;;;;;;;;::         ..  .    ,,:::::;::  ,:::;:.
               :;;::::,:;;;;;;;;;;;;:;;::;;:,:::,:,;;;;::;;;;;;;;,     ...., .  :::;;;;;;:::::::.::;
          :;::::::;::::::::::::::::,::::,:;;;;;;;;;;;;;;;;;;;;;;;:;     ..:.,.  .:.:;.:;:  :,,;;::::
      :;:;:;;;;::::::::;;;;;;;;;;;;;;;;;;:::;;;;;;;;;;;;;;;;;::;;:.::;;,,...:,   ::;,.;,,::,:;::::;;
            :::::::::::;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:;;::;,;;::;;:.:;.    :;:,,:::,:;;;;;;:
,,,,,,,.. ,,;;;;;;;;;;;;;;;;;;;;;;;;;;;;:;:,,;;;;;;;;;;;;;;;::;::;,,,;:;:;;:;;;;,:::;:::::,:,:,,..::
,,,::::::::,;;;;;;;;;;;;;;;;;::;;;;;;;:;;:::;::;;;;;;;;;;:::::,::::::::;;:;:;;;;;;;;;;::;;::;::;;:;:
:::::::;:;;;;;;;;;;;;;;;;;;;;;:::::::::;;;;;;;;;;;::;;;;;;;;;;;:;;;;;;:;;;;:;;;;;,:;:;;;:::;::;;:;;;
:,,:::::::::;:::;:::;;;:::::::::::::;;:::::::;;:::;;;;;;;;;;,::;,;,;;;;;;::;;;;;:::;:;:;::;;:;;;:;;;
::::::::,::;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:;:;;;;;;;:::;;;::;;;:;:::;:
::::::::::;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::;;;;;;;:;;;;;;;;;;::;;:::;;;;:;;,
;;;;::::::;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:;;;;;;;;;;,,,,,,:;:;:,:,,;::::::;;;;;:;;;;:;;;;;;;;:;;;;;
;;;;;;:::::;;;;;;;:;;;;;:;;;;;;;;;;;;;;;:;,;:;;;;;;:;;,,,,,,,,,,:;;;:::::::;;;;;:,:;;:::;:;:;;;:;;;:
:;;;;;:::;::;:;;:;;;;;;;;;;;;;;;;;;;;;:;:,:,,,,:;::,;:;;;;:;:,,,,,,,,,,,,,,,.;:;,;;::;:,;;:::;;;;;;;
;;;;;;;;;:::,;;;;;::,::;;:;;;;;;;;;;::::,,,:::;,;::,,::::;:;:,::,,:,,..;,.:,,.,,,;;;;;;;;;;;;;;;;;;;
;;;;:;;;::;::::;;;;;::::;;:;;;;;;;:;;::::::..:::;:::::::.:;,:;.:,:.::::,,.:,;;,::;;,;;;;;;;:;;;;;;;;
```

$ txtr -c "松本城　" -w 100 -f 0.3 assets/castle.jpg

```
本城　城松　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　　　　　　　　松　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　　　　　　　本松松松松　　　松　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　　　　　　　本本松松松松松松松　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　　　　　松本松松松本松松松松松松松　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　　城松本松松松松松松本本本松松松松本松　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　本松本松松松本本本本本松松松松松本松松本本松本本松松　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　城本本本本本本本本松松松松松松本本本本本松松松本　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　本松松松松松松松松松松松松松松松松松松松松本　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　　本松松本本松松松松松松松松松松本松松松松松本　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　本松松松松本松松本松松本松松松松松本松松本本松松松松松松　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　本本本本松松松松本城城本城本松本松松松松本本松松松松松松松松本松松松　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　本城松松本本本本本本松松松松松松松松松松松松松本本　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　　　　　　松松松松松松松松松松松松松松松松松松松松松松松松本本　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　
　　　　　　　　　　　　　　　松松本松松松松松松松松松松松松本松本松本松松松松松松松松松松松松本　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　本
　　　　　　　　　　城松松松松松松本本本本松本城松松松松松松松松松松本本松松松本本松本松松本本松松松松松松　　　　　　　　　　　　　　　　　　　　　　　松松本松
　　　　　　　　　　　　　　　本松松松松松本本城本本松松松松松松松松松城松本城松松本松松本本松本本松　　　　　　　　　　　　　　　　　　　　　　　　本本本本本本
　　　　　　　　　　　　　　　松松松本本本松松松松松松本松本松松松本松本本松本本松　松松松松松松松本城　　　　　　　　　　　　　　本本本本本本松本　　城本松本本
　　　　　　　　　　　　松松松松松本本松松松松松松松松松本松松本松松城本松松本本松松松松松松松松松松松松本本　　　　城城　　　　城本本松松松松本本本本松本　城松
　　　　　城本松本本本本本本本本本本本本本本本本松松松松松本松松松松松松松松松本本本松松松松松松松松松松松松松　　城　　城本　　　　本松城松本本本城　城松本松松
　　　　　　　　松本松松松松松松松松松松松松松本本本本本松松松松本松松松松松松松松松松松松松松松松松松松城本松松松松本　城本　　　　本松松城本本本本本本松本松松
本本本本本　　　　松松松松松松松松松松松松松松松松松松松松松松松本松松松松本本本松松松松松松松松松松松松松松松松本松松松松松松松本城城松城本松本城本城本本　城松
本本本本本本本本本松松松松松松松松松松松松松松松松松松松松松本松本本本松松松松松松松本本本本本本松本本本本本松松本松松松松松松松松松松松松本松本本本松松松松松松
本本本本本松松松松松松本本本本本本本本本松松本松松松松松松松松松松本本本松松松本本本松松松松松松松松本松松松松松松松松松松松城松松松松松松松本本松松本本本松松松
本本本本本本本本本本本本本本本本松松本本本本本本本松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松本本松松松本本松松松松本松松
本松本本本本本本松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松松本松松本松松松松松松城
松松松松本松本本本松松松松松松松松松松松松松松松松松松松松松松松松本松松松松松松松松松本本本本本松松本本本本松本本本本松松松松松本本松松松松松松本松松松松松松松
松松松松松本本松松松松松松松松松松松松松松松松松松松松松松松松松本本本城松松松松松松松本本本本本本松松松松松松松松松松松松松松松本松松松本本松城松松城松松松松松
松松松松松本松本本松本松本松本本本本松松松松松松松松松松松松本本松本本松本松本松本松本本松本本本松本本本本本本城本城本城城本本城本松松松本松松松松松松本松松松松
松松松松松本松本松本本本松松松松本本松松松松松松松松松松松松松本城城城城本本松本本本本本城本本本本本城本本本松本本本本城城松城本松松松松城松松松松松松松松松松松
```

$ txtr -w 40  -f 0.3 -i assets/castle.jpg

```
###%{#############################%%%%%%
##%{##########:#####################%%%%
##############..,.####################%%
############:...:.,{####################
#########;,.,,::;;,.:..#################
###########.....,....###################
#######/.,,..,,.....:.,,################
##########...........;,#################
#####;.,,::;...:,..,;,...#############;:
#######...;..,...,,;.,.,.#######{#;%#{;:
####{,,.,...:,..,::,...,...##:%#%,;,{.,:
#####:::...............,..,,,..;#::%%,.,
:::.,.........,..,......,,..,..:.,,,..,.
:,::.............................,.,.:,.
.,.,,..,........;/,,./;;;;,,::..:....,.,
...,,.,:,.....:,:../;,,,::,::///....;,..
```

$ txtr -w 40 -e red -f 0.3 assets/castle.jpg

```
...,:...........................,,,,,,,,
...:..........{....................,,,,,
..............%%{%...................,,,
............/##%{#%:....................
........./{%%{//;/{%{%%.................
...........#####{%##%...................
.......;%%%%%%%#%%%%/#%%................
..........%%%#%#####%/{.................
.....;%{{{{/#%%/{%%{;%#%%.............;/
.......%%%/##%###%%;%%#{%.......,.;,.://
....,{%%%%%%{{%%{//%%##%%#%../..,{/%,%{{
.....{{{###############%%#%%%%%/./{,,{%{
{{{%{%######%%%%%%%%%%%%{{%%%%#/#%%%%%%%
{{{{###%######%########%%%%%%%###{%%%{{#
%{%{{%%{#%%%#%#%;;%%%;////%{{{##{%###{%{
##%{{%{/{%%%%%/{/%%:;{{{//{//:,:####;{%%
```


$ txtr -w 40 -e blue -f 0.3 assets/castle.jpg

```
........................................
..............;.........................
..............{%{%......................
............/%%%/%/,....................
.........;%%%{{/;/%%{%%.................
...........%%%%%{%%%%...................
.......:%{{%%%{#%%%%/%{{................
..........%%%%%##%#%%;{.................
.....;%{{//;%%%/{%%{;%%%%.............;/
.......%%%;#%{%#%{{;%{#{%.......,./..,/{
....,{{%%%%{/{%%{{/%%#%%%%%../...{/%,%%%
.....{{{%#############%{{%{%{%%/.{{..%#%
;;/%{%%%%%%%%%%%%%%%%#%%%%###%#/#%%{%%{%
////%%#%%%%%%%%#%%%%#######%#%#####%#%%#
#%%%{%%{%%%%%%%%;;{%%:;;;/%{/;##{%###%%%
##%%%#%{{%%%%%{%{%#;/%{%{{%{{;;:%#%#/%%%
```

$ txtr -w 40 -e green -f 0.3 assets/castle.jpg

```
....,...................................
...,........../.........................
..............%%{%......................
............/%%%/%{,....................
.........;{%%{//;;{%/%%.................
...........#####{%%#%...................
.......:%{{%%%%#%%%%/%%%................
..........%%%#%#####%;{.................
.....;%{{{/;#%%/{%%{;%#%%.............;/
.......%%%;#%{%#%{%;%%#{%.......,.;..,;/
....,{%%%%%%/{%%{//%%##%%%%../..,{;{,%//
.....{{{###############%%%{%{%%/.//..{%{
///%{%%%%%%%%%%%%%%%%%%%{{%%{%#/#%{{%%{%
/{//###%##%%##%##%#%#######%%%##%{%%#/{#
%{%{{%%{#%%%%%%%;;{%%;;;;/%{//##/%##%{%{
##%%%%{{{%%%%%/{/%%;;{{{{/%{/;::%###;{%%
```