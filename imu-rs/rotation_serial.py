from math import cos, sin, pi
from turtle import pos
import serial
import json
import time

### chip is rotated so X is Z axis
### robot forward is Z, robot sideways is Y
ROTATION_AXIS = "Z"
FORWARD_AXIS = "X"

dt = 0
start = time.time()
elapsed = 0
angle = 0

def integrate_rotation(data, dt):
    angular_z = data["G"][ROTATION_AXIS]
    return angular_z * dt

ser = serial.Serial('/dev/ttyACM0', baudrate=921600)  # open serial port
ser.baudrate = 921600
print(ser.name)         # check which port was really used

start = time.time()
while True:

    imu_string = ser.readline().decode('utf8')
    imu_data = {}

    try: 
        imu_data = json.loads(imu_string)
    except:
        print("String not JSON format: %s"%imu_string)

    if "G" in imu_data:
        dt = time.time() - start # time since last measurement
        elapsed += dt
        angle = angle + integrate_rotation(imu_data, dt)
        print(angle)
        start = time.time()
        
ser.close()             # close port