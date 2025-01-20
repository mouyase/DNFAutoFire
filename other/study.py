import interception  
from pynput import mouse, keyboard  
  
  
# key_state = {  
#     'q' : False,  
#     'w' : False,  
#     'e' : False,  
#     'r' : False,  
#     's' : False,  
#     'space' : False,  
# }  
# interception.auto_capture_devices(keyboard=True, mouse=True, verbose=True)  
  
# def on_press(key):  
#     global key_state  
#     if key == keyboard.Key.space:  
#         key_state['space'] = True  
  
#     if key_state['space']:  
#         if key == keyboard.KeyCode(char='w') and key_state['w'] == False:  
#             key_state['w'] = True  
#             print("wsq 连招")  
#             interception.press('s')  
#             interception.press('q')  
  
  
#         if key == keyboard.KeyCode(char='r') and key_state['r'] == False:  
#             key_state['r'] = True  
#             print("rsq 连招")  
#             interception.press('q')  
  
  
# def on_release(key):  
#     global key_state  
#     if key == keyboard.Key.space:  
#         key_state['space'] = False  
  
#     if key == keyboard.KeyCode(char='w'):  
#         key_state['w'] = False  
  
#     if key == keyboard.KeyCode(char='r'):  
#         key_state['r'] = False  
  
  
# with keyboard.Listener(on_release=on_release, on_press=on_press) as listener:  
#     listener.join()

