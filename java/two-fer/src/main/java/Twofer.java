class Twofer {
    public String twofer(final String name) {
        return "One for "
                + (name == null ? "you" : name)
                + ", one for me.";
    }
}
