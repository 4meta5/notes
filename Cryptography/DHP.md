<style TYPE="text/css">
code.has-jax {font: inherit; font-size: 100%; background: inherit; border: inherit;}
</style>
<script type="text/x-mathjax-config">
MathJax.Hub.Config({
    tex2jax: {
        inlineMath: [['$','$'], ['\\(','\\)']],
        skipTags: ['script', 'noscript', 'style', 'textarea', 'pre'] // removed 'code' entry
    }
});
MathJax.Hub.Queue(function() {
    var all = MathJax.Hub.getAllJax(), i;
    for(i = 0; i < all.length; i += 1) {
        all[i].SourceElement().parentNode.className += ' has-jax';
    }
});
</script>
<script type="text/javascript" src="http://cdn.mathjax.org/mathjax/latest/MathJax.js?config=TeX-AMS-MML_HTMLorMML"></script>
# Diffie-Hellman Problem (DHP)
> The motivation for this problem is that many security systems use one-way functions: mathematical functions that are fast to compute, but hard to reverse i.e. enable encrypting a message, but reversing the encryption (without the decryption key) is difficult. If solving DHP were easy, these systems would be broken. 

The **Diffie-Hellman Problem (DHP)** is stated informally as:<br>
Given an element `$g$` amd the values of `$g^{x}$` amd `$g^{y}$`, what is the value of `$g^{xy}$`?<br>
`$g$` is a generator of some group and `$x$` and `$y$` are randomly chosen integers.


