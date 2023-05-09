from math import cos, sin, pi
from turtle import pos
import serial
import json
import time
from datetime import datetime
import numpy as np
from sklearn.linear_model import LinearRegression

### chip is rotated so X is Z axis
### robot forward is Z, robot sideways is Y
ROTATION_AXIS = "Z"
FORWARD_AXIS = "X"

IMU_HZ = 1000.0
dt = 1 / IMU_HZ
epsilon = 0.005
# print(dt)
start = time.time()
elapsed = 0

coeff = 0
bias = 0

def integrate_rotation(data, dt):
    angular_z = data["G"][ROTATION_AXIS]
    return angular_z * dt

def integrate_velocity(data, dt):
    acc = data["A"][FORWARD_AXIS] * 9.81 #g's to m/s^2
    print(acc * dt)
    return acc * dt

def integrate_position(position, angle, velocity, dt):
    dvel = velocity * dt
    rad = angle * pi / 180.0
    position[0] += cos(rad) * dvel
    position[1] += sin(rad) * dvel
    return position    

ser = serial.Serial('/dev/ttyACM0', baudrate=921600)  # open serial port
ser.baudrate = 921600
print(ser.name)         # check which port was really used

velocity = 0

velocity_measurements = []
time_measurements = []

# get first 3s of data
start = time.time()
while True:

    imu_string = ser.readline().decode('utf8')
    imu_data = {}

    try: 
        imu_data = json.loads(imu_string)
    except:
        print("String not JSON format: %s"%imu_string)
        elapsed = 0

    if "G" in imu_data:
        dt = time.time() - start # time since last measurement
        elapsed += dt
        a = imu_data["A"][FORWARD_AXIS] * 9.81 #g's to m/s^2
        velocity += a * dt
        # velocity = a
        print(a)
        velocity_measurements.append(velocity)
        time_measurements.append(elapsed)
        start = time.time()

    if elapsed > 3.0:
        break
        
# get line of best fit
# linear regression blah blah blah
time_measurements = np.array(time_measurements, dtype=np.float32).reshape((-1, 1)) # X
velocity_measurements = np.array(velocity_measurements, dtype=np.float32) # Y
model = LinearRegression().fit(time_measurements, velocity_measurements)
r_sq = model.score(time_measurements, velocity_measurements)
bias = model.intercept_
coef = model.coef_[0]

print(r_sq)
print(bias)
print(coef)

# exit()

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

    if "G" in imu_data:
        dt = time.time() - start # time since last measurement
        elapsed += dt
        angle = angle + integrate_rotation(imu_data, dt)
        velocity = velocity + integrate_velocity(imu_data, dt)
        corrected_vel = velocity - ((elapsed * coeff) + bias)
        print(corrected_vel)
        # position = integrate_position(position, angle, corrected_vel, dt)
        start = time.time()
        # print(position)
        
ser.close()             # close port