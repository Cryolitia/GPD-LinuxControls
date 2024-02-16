# GPD LinuxControls

This is a reverse engineered and reference implementation of GPD WinControls, developed and tested on GPD Win Max 2 2023.

> [!CAUTION]
> 
> This repository is the product of reverse engineering
>
> USB data frames sent to the device will be written **directly** to the EC ROM. Wrong speculation and implementation during the reverse engineering process may irreversibly destroy your device.
>
> This repository and its contributors do not make any commitments, and **all consequences are at the user's own risk**.

## GPD WinControls Protocal

### Basic Infomation

- idVendor: 0x2F24
- idProduct: 0x0135

All request commands should be sent by `SET_REPORT`, and read the response by `GET_REPORT`.

|               | SET_REPORT | GET_REPORT |
| ------------- | ---------- | ---------- |
| bmRequestType | 0x21       | 0xa1       |
| bRequest      | 0x09       | 0x01       |
| wValue        | 0x0201     | 0x0101     |
| wIndex        | 2          | 2          |
| wLength       | 33         | 65         |

### Keyboard-Mouse

Keyboard-Mouse follows [HID Usage ID](https://download.microsoft.com/download/1/6/1/161ba512-40e2-4cc9-843a-923143f3456c/translate.pdf)

In addition, there are GPD defined HID Usage ID extensions:
- `0xE8`: Mouse_UP
- `0xE9`: Mouse_Down

### `VIBRATE`

- `0`: disable
- `1`: light
- `2`: heavy

### Dead Zones

In range [-10, 10] ([0xF6, 0x0A])

### Back Button Delay

- `0x00`: 0 ms
- `0x64`: 100 ms

### Header

<table>
    <tr>
        <td align="right">0</td>
        <td align="right">1</td>
        <td align="right" colspan="2">2</td>
        <td align="right">3</td>
        <td align="right">4</td>
        <td align="right">5</td>
        <td align="right">6</td>
        <td align="right">7</td>
    </tr>
    <tr>
        <td colspan="2">01a5</td>
        <td>R/W</td>
        <td>MAJOR_SERIAL</td>
        <td>5a</td>
        <td>CHECK_DIGIT</td>
        <td>00</td>
        <td>MINOR_DIGIT</td>
        <td>00</td>
    </tr>
</table>

1. `R/W`
    - R: `1`
    - W: `2`
1. `MAJOR_SERIAL`
    - `R/W` = `1`: In range [0, 2]
    - `R/W` = `2`: In range [1, 3]
1. `CHECK_DIGIT` + `Header[2]` = `0xFF`
1. `MINOR_SERIAL`
    - `MAJOR_SERIAL` = `1`
        - `R/W` = `1`: In range [0, 3]
        - `R/W` = `2`: In range [0, 7]
    - Else: `0`

### `R/W` = `1`

#### `MAJOR_SERIAL` = `0`

##### Request

Command: `01a5105aef00000000000000000000000000000000000000000000000000000000`

##### Response

<table>
    <tr>
        <td align="right">0-8</td>
        <td align="right">9</td>
        <td align="right">10</td>
        <td align="right">11</td>
        <td align="right">12</td>
        <td align="right">13-63</td>
    </tr>
    <tr>
        <td></td>
        <td colspan="2">GPD Mainboard(GamePad) Firmware</td>
        <td colspan="2">GPD Extendboard(Keyboard-Mouse) Firmware</td>
        <td></td>
    </tr>
    <tr>
        <td>01a5105aef000000aa</td>
        <td>MAJOR_VERSION</td>
        <td>MINOR_VERSION</td>
        <td>MAJOR_VERSION</td>
        <td>MINOR_VERSION</td>
        <td>000000000000000000000000000000e40200000000000000000000000000000000000000000000000000000000000000000000</td>
    </tr>
</table>

#### `MAJOR_SERIAL` = `1`

##### `MINOR_SERIAL` = `0`

###### Request

Command: `01a5115aee00000000000000000000000000000000000000000000000000000000`

###### Response

<table>
    <tr>
        <td align="right">0</td>
        <td align="right">1</td>
        <td align="right">2</td>
        <td align="right">3</td>
        <td align="right">4</td>
        <td align="right">5</td>
        <td align="right">6</td>
        <td align="right">7</td>
        <td align="right">8</td>
        <td align="right">9</td>
        <td align="right">10</td>
        <td align="right">11</td>
        <td align="right">12</td>
        <td align="right">13</td>
        <td align="right">14</td>
        <td align="right">15</td>
        <td align="right">16</td>
        <td align="right">17</td>
        <td align="right">18</td>
        <td align="right">19</td>
        <td align="right">20</td>
        <td align="right">21</td>
        <td align="right">22</td>
        <td align="right">23</td>
        <td align="right">24</td>
        <td align="right">25</td>
        <td align="right">26</td>
        <td align="right">27</td>
        <td align="right">28-49</td>
        <td align="right">50</td>
        <td align="right">51</td>
        <td align="right">52</td>
        <td align="right">53</td>
        <td align="right">54</td>
        <td align="right">55</td>
        <td align="right">56</td>
        <td align="right">57</td>
        <td align="right">58</td>
        <td align="right">59</td>
        <td align="right">60</td>
        <td align="right">61</td>
        <td align="right">62</td>
        <td align="right">63</td>
    </tr>
    <tr>
        <td colspan=28>Keyboard-Mouse</td>
        <td rowspan=3>000000000000ea00eb00ec00ed000000000000000000</td>
        <td colspan=14>Back Button</td>
    </tr>
    <tr>
        <td colspan=8>Directional Pad</td>
        <td rowspan=2>A Button</td>
        <td rowspan=2>00</td>
        <td rowspan=2>B Button</td>
        <td rowspan=2>00</td>
        <td rowspan=2>X Button</td>
        <td rowspan=2>00</td>
        <td rowspan=2>Y Button</td>
        <td rowspan=2>00</td>
        <td colspan=10>Left Stick</td>
        <td colspan=2>Right Stick</td>
        <td rowspan=2>LEFT_1</td>
        <td rowspan=2>00</td>
        <td rowspan=2>LEFT_2</td>
        <td rowspan=2>00</td>
        <td rowspan=2>LEFT_3</td>
        <td rowspan=2>00</td>
        <td rowspan=2>LEFT_4</td>
        <td rowspan=2>00</td>
        <td rowspan=2>RIGHT_1</td>
        <td rowspan=2>00</td>
        <td rowspan=2>RIGHT_2</td>
        <td rowspan=2>00</td>
        <td rowspan=2>RIGHT_3</td>
        <td rowspan=2>00</td>
    </tr>
    <tr>
        <td>UP</td>
        <td>00</td>
        <td>DOWN</td>
        <td>00</td>
        <td>LEFT</td>
        <td>00</td>
        <td>RIGHT</td>
        <td>00</td>
        <td>UP</td>
        <td>00</td>
        <td>DOWN</td>
        <td>00</td>
        <td>LEFT</td>
        <td>00</td>
        <td>RIGHT</td>
        <td>00</td>
        <td>PUSH</td>
        <td>00</td>
        <td>PUSH</td>
        <td>00</td>
    </tr>
</table>

##### `MINOR_SERIAL` = `1`

###### Request

Command: `01a5115aee00010000000000000000000000000000000000000000000000000000`

###### Response

<table>
    <tr>
        <td align="right">0</td>
        <td align="right">1</td>
        <td align="right">2</td>
        <td align="right">3-7</td>
        <td align="right">8</td>
        <td align="right">9</td>
        <td align="right">10</td>
        <td align="right">11</td>
        <td align="right">12-15</td>
        <td align="right">16</td>
        <td align="right">17</td>
        <td align="right">18</td>
        <td align="right">19</td>
        <td align="right">20</td>
        <td align="right">21</td>
        <td align="right">22</td>
        <td align="right">23</td>
        <td align="right">24</td>
        <td align="right">25</td>
        <td align="right">26</td>
        <td align="right">27</td>
        <td align="right">28</td>
        <td align="right">29</td>
        <td align="right">30</td>
        <td align="right">31</td>
        <td align="right">32-63</td>
    </tr>
    <tr>
        <td colspan=2>Back Button</td>
        <td rowspan=2>VIBRATE</td>
        <td rowspan=2>0000ff0000</td>
        <td colspan=4>Stick Dead Zones</td>
        <td></td>
        <td colspan=6>Back Button Delay</td>
        <td rowspan=2>2c</td>
        <td rowspan=2>01</td>
        <td colspan=6>Back Button Delay</td>
        <td rowspan=2>2c</td>
        <td rowspan=2>01</td>
        <td rowspan=2>0000000000000000000000000000000000000000000000000000000000000000</td>
    </tr>
    <tr>
        <td>RIGHT_4</td>
        <td>00</td>
        <td>LEFT_CENTER</td>
        <td>LEFT_BORDER</td>
        <td>RIGHT_CENTER</td>
        <td>RIGHT_BORDER</td>
        <td>00000000</td>
        <td>LEFT_1</td>
        <td>00</td>
        <td>LEFT_2</td>
        <td>00</td>
        <td>LEFT_3</td>
        <td>00</td>
        <td>RIGHT_1</td>
        <td>00</td>
        <td>RIGHT_2</td>
        <td>00</td>
        <td>RIGHT_3</td>
        <td>00</td>
    </tr>
</table>

##### `MINOR_SERIAL` = `2`

###### Request

Command: `01a5115aee00020000000000000000000000000000000000000000000000000000`

###### Response

`00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000`

##### `MINOR_SERIAL` = `3`

###### Request

Command: `01a5115aee00030000000000000000000000000000000000000000000000000000`

###### Response

`00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000`

#### `MAJOR_SERIAL` = `2`

##### Request

Command: `01a5125aed00000000000000000000000000000000000000000000000000000000`

##### Response

<table>
    <tr>
        <td align=right>0-7</td>
        <td align=right>8-23</td>
        <td align=right>24-29</td>
        <td align=right>30-63</td>
    </tr>
    <tr>
        <td>Header</td>
        <td rowspan=2>aa031401230000000000000000000000</td>
        <td rowspan=2>CHECKSUM</td>
        <td rowspan=2>00000000000000000000000000000000000000000000000000000000000000000000</td>
    </tr>
    <tr>
        <td>01a5125aed000000</td>
    </tr>
</table>

### `R/W` = `2`

Write commands usually has no response.

#### `MAJOR_SERIAL` = `1`

##### `MINOR_SERIAL` = `0`

<table>
  <tr>
    <td align="right">0-7</td>
    <td align="right">8</td>
    <td align="right">9</td>
    <td align="right">10</td>
    <td align="right">11</td>
    <td align="right">12</td>
    <td align="right">13</td>
    <td align="right">14</td>
    <td align="right">15</td>
    <td align="right">16</td>
    <td align="right">17</td>
    <td align="right">18</td>
    <td align="right">19</td>
    <td align="right">20</td>
    <td align="right">21</td>
    <td align="right">22</td>
    <td align="right">23</td>
    <td align="right">24-32</td>
  </tr>
  <tr>
    <td rowspan=2>Header</td>
    <td colspan=16>Keyboard-Mouse</td>
    <td rowspan=3>000000000000000000</td>
  </tr>
  <tr>
    <td colspan=8>Directional Pad</td>
    <td rowspan=2>A Button</td>
    <td rowspan=2>00</td>
    <td rowspan=2>B Button</td>
    <td rowspan=2>00</td>
    <td rowspan=2>X Button</td>
    <td rowspan=2>00</td>
    <td rowspan=2>Y Button</td>
    <td rowspan=2>00</td>
  </tr>
  <tr>
    <td>01a5215ade000000</td>
    <td>UP</td>
    <td>00</td>
    <td>DOWN</td>
    <td>00</td>
    <td>LEFT</td>
    <td>00</td>
    <td>RIGHT</td>
    <td>00</td>
  </tr>
</table>

##### `MINOR_SERIAL` = `1`

<table>
  <tr>
    <td align=right>0-7</td>
    <td align=right>8</td>
    <td align=right>9</td>
    <td align=right>10</td>
    <td align=right>11</td>
    <td align=right>12</td>
    <td align=right>13</td>
    <td align=right>14</td>
    <td align=right>15</td>
    <td align=right>16</td>
    <td align=right>17</td>
    <td align=right>18</td>
    <td align=right>19</td>
    <td align=right>20-32</td>
  </tr>
  <tr>
    <td rowspan=2>Header</td>
    <td colspan=12>Keyboard-Mouse</td>
    <td rowspan=3>00000000000000000000000000</td>
  </tr>
  <tr>
    <td colspan=10>Left Stick</td>
    <td colspan=2>Right Stick</td>
  </tr>
  <tr>
    <td>01a5215ade000100</td>
    <td>UP</td>
    <td>00</td>
    <td>DOWN</td>
    <td>00</td>
    <td>LEFT</td>
    <td>00</td>
    <td>RIGHT</td>
    <td>00</td>
    <td>PUSH</td>
    <td>00</td>
    <td>PUSH</td>
    <td>00</td>
  </tr>
</table>

##### `MINOR_SERIAL` = `2`

`01a5215ade0002000000ea00eb00ec00ed00000000000000000000000000000000`

##### `MINOR_SERIAL` = `3`

<table>
  <tr>
    <td align=right>0-7</td>
    <td align=right>8-9</td>
    <td align=right>10</td>
    <td align=right>11</td>
    <td align=right>12</td>
    <td align=right>13</td>
    <td align=right>14</td>
    <td align=right>15</td>
    <td align=right>16</td>
    <td align=right>17</td>
    <td align=right>18</td>
    <td align=right>19</td>
    <td align=right>20</td>
    <td align=right>21</td>
    <td align=right>22</td>
    <td align=right>23</td>
    <td align=right>24-32</td>
  </tr>
  <tr>
    <td>Header</td>
    <td rowspan=2>0000</td>
    <td colspan=14>Back Button</td>
    <td rowspan=2>000000000000000000</td>
  </tr>
  <tr>
    <td>01a5215ade000300</td>
    <td>LEFT_1</td>
    <td>00</td>
    <td>LEFT_2</td>
    <td>00</td>
    <td>LEFT_3</td>
    <td>00</td>
    <td>LEFT_4</td>
    <td>00</td>
    <td>RIGHT_1</td>
    <td>00</td>
    <td>RIGHT_2</td>
    <td>00</td>
    <td>RIGHT_3</td>
    <td>00</td>
  </tr>
</table>

##### `MINOR_SERIAL` = `4`

<table>
  <tr>
    <td align=right>0-7</td>
    <td align=right>8</td>
    <td align=right>9</td>
    <td align=right>10</td>
    <td align=right>11-15</td>
    <td align=right>16</td>
    <td align=right>17</td>
    <td align=right>18</td>
    <td align=right>19</td>
    <td align=right>20-32</td>
  </tr>
  <tr>
    <td>Header</td>
    <td colspan=2>Back Button</td>
    <td rowspan=2>VIBRATE</td>
    <td rowspan=2>0000ff0000</td>
    <td colspan=4>Stick Dead Zones</td>
    <td rowspan=2>00000000000000000000000000</td>
  </tr>
  <tr>
    <td>01a5215ade0004</td>
    <td>RIGHT_4</td>
    <td>00</td>
    <td>LEFT_CENTER</td>
    <td>LEFT_BORDER</td>
    <td>RIGHT_CENTER</td>
    <td>RIGHT_BORDER</td>
  </tr>
</table>

##### `MINOR_SERIAL` = `5`

<table>
  <tr>
    <td align=right>0-7</td>
    <td align=right>8</td>
    <td align=right>9</td>
    <td align=right>10</td>
    <td align=right>11</td>
    <td align=right>12</td>
    <td align=right>13</td>
    <td align=right>14</td>
    <td align=right>15</td>
    <td align=right>16</td>
    <td align=right>17</td>
    <td align=right>18</td>
    <td align=right>19</td>
    <td align=right>20</td>
    <td align=right>21</td>
    <td align=right>22</td>
    <td align=right>23</td>
    <td align=right>24-32</td>
  </tr>
  <tr>
    <td>Header</td>
    <td colspan=6>Back Button Delay</td>
    <td rowspan=2>2c</td>
    <td rowspan=2>01</td>
    <td colspan=6>Back Button Delay</td>
    <td rowspan=2>2c</td>
    <td rowspan=2>01</td>
    <td rowspan=2>000000000000000000</td>
  </tr>
  <tr>
    <td>01a5215ade000500</td>
    <td>LEFT_1</td>
    <td>00</td>
    <td>LEFT_2</td>
    <td>00</td>
    <td>LEFT_3</td>
    <td>00</td>
    <td>RIGHT_1</td>
    <td>00</td>
    <td>RIGHT_2</td>
    <td>00</td>
    <td>RIGHT_3</td>
    <td>00</td>
  </tr>
</table>

##### `MINOR_SERIAL` = `6`

`01a5215ade00060000000000000000000000000000000000000000000000000000`

##### `MINOR_SERIAL` = `7`

`01a5215ade00070000000000000000000000000000000000000000000000000000`

#### `MAJOR_SERIAL` = `2`

##### Request

`01a5225aed00000000000000000000000000000000000000000000000000000000`

##### Response

<table>
    <tr>
        <td align=right>0-7</td>
        <td align=right>8-23</td>
        <td align=right>24-29</td>
        <td align=right>30-63</td>
    </tr>
    <tr>
        <td>Header</td>
        <td rowspan=2>aa031401230000000000000000000000</td>
        <td rowspan=2>CHECKSUM</td>
        <td rowspan=2>00000000000000000000000000000000000000000000000000000000000000000000</td>
    </tr>
    <tr>
        <td>01a5225aed000000</td>
    </tr>
</table>

#### `MAJOR_SERIAL` = `3`

`01a5235adc00000000000000000000000000000000000000000000000000000000`

This command will save all config into ROM.
