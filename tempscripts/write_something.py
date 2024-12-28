from pynput.keyboard import Key, Controller
from time import sleep

keyboard = Controller()

lorem_ipsum = """Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Duis sapien nunc, commodo et, interdum suscipit, sollicitudin et, dolor. Curabitur sagittis hendrerit ante. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. In dapibus augue non sapien. Et harum quidem rerum facilis est et expedita distinctio. Cras elementum. Fusce aliquam vestibulum ipsum. Mauris dolor felis, sagittis at, luctus sed, aliquam non, tellus. Vestibulum fermentum tortor id mi. Vivamus luctus egestas leo. In laoreet, magna id viverra tincidunt, sem odio bibendum justo, vel imperdiet sapien wisi sed libero. Phasellus et lorem id felis nonummy placerat. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Pellentesque arcu. Etiam ligula pede, sagittis quis, interdum ultricies, scelerisque eu.

Maecenas sollicitudin. Aliquam ante. In dapibus augue non sapien. Nullam feugiat, turpis at pulvinar vulputate, erat libero tristique tellus, nec bibendum odio risus sit amet ante. Mauris elementum mauris vitae tortor. Mauris metus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Nullam rhoncus aliquam metus. Phasellus et lorem id felis nonummy placerat. Quisque tincidunt scelerisque libero. Maecenas ipsum velit, consectetuer eu lobortis ut, dictum at dui. Etiam quis quam. In rutrum. In dapibus augue non sapien.
"""

list_of_ipsum = lorem_ipsum.split()

print("Starting wait for 15 seconds....")
sleep(5)
print("Starting lorem ipsum")
      

for ch in lorem_ipsum:
    keyboard.press(ch)
    print(f"Pressing {ch}")
    keyboard.release(ch)
