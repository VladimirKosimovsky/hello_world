def add_suffix(words, suffix):
    return [word + suffix for word in words]

# vs closure
def suffixer(suffix):
    def add(word):
        return word + suffix
    return add

add_ing = suffixer("ing")
print(add_ing("run"))  # running
