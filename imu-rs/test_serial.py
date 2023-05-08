from cmath import sin
from math import cos, pi
import serial
import json
import time
from datetime import datetime

### chip is rotated so X is Z axis
### robot forward is Z, robot sideways is Y

IMU_HZ = 1000.0
dt = 1 / IMU_HZ
# print(dt)
start = time.time()

def integrate_rotation(data, dt):
    angular_z = data["GYRO"]["Y"]
    return angular_z * dt

def integrate_velocity(data, dt):
    forward = data["ACCEL"]["Z"]
    return forward * dt

def integrate_position(angle, velocity, dt):
    dvel = velocity * dt
    rad = angle * pi / 180.0
    return [cos(rad) * dvel, sin(rad) * dvel]
    

ser = serial.Serial('/dev/ttyACM0', baudrate=921600)  # open serial port
ser.baudrate = 921600
print(ser.name)         # check which port was really used

angle = 0
velocity = 0
position = [0,0]

start = time.time()

while True:

    imu_string = ser.readline().decode('utf8')
    imu_data = {}

    try: 
        imu_data = json.loads(imu_string)
    except:
        print("String not JSON format: %s"%imu_string)

    if "GYRO" in imu_data:
        dt = time.time() - start # time since last measurement
        angle = angle + integrate_rotation(imu_data, dt)
        velocity = velocity + integrate_velocity(imu_data, dt)
        position = integrate_position(angle, velocity, dt)
        start = time.time()
        
ser.close()             # close port